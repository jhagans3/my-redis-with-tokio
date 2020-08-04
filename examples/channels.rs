use bytes::Bytes;
use mini_redis::client;
use tokio::sync::{mpsc, oneshot};

/// Provided by the requester and used by the manager task to send the command
/// response back to the requester.
// oneshot - single-producer, single consumer channel. A single value can be sent.
type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

/// Multiple different commands are multiplexed over a single channel.
#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Vec<u8>,
        resp: Responder<()>,
    },
}

// mini-redis-server
// cargo run --example channels
#[tokio::main]
async fn main() {
    // mpsc - multi-producer, single-consumer channel. Many values can be sent
    let (mut tx, mut rx) = mpsc::channel(32);
    // The `Sender` handles are moved into the tasks. As there are two
    // tasks, we need a second `Sender`.
    // Clone a `tx` handle for the second f
    let mut tx2 = tx.clone();

    // The `move` keyword is used to **move** ownership of `rx` into the task.
    let manager = tokio::spawn(async move {
        // Open a connection to the mini-redis address.
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();

        // Start receiving messages
        while let Some(cmd) = rx.recv().await {
            match cmd {
                Command::Get { key, resp } => {
                    let res = client.get(&key).await;
                    // Calling send on oneshot::Sender completes
                    // immediately and does not require an `.await.`
                    // This is because send on an oneshot channel will
                    // always fail or succeed immediately without any form of waiting.
                    // Ignore errors
                    let _ = resp.send(res);
                }
                Command::Set { key, val, resp } => {
                    let res = client.set(&key, val.into()).await;
                    // Sending a value on a oneshot channel
                    // returns Err when the receiver half has dropped.
                    // This indicates the receiver is no longer interested in
                    // the response. In our scenario, the receiver cancelling
                    // interest is an acceptable event. The Err returned by
                    // resp.send(...) does not need to be handled.
                    // Ignore errors
                    let _ = resp.send(res);
                }
            }
        }
    });

    // Spawn two tasks, each setting a value
    // one gets a key, the other sets a key
    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Get {
            key: "hello".to_string(),
            resp: resp_tx,
        };

        // Send the GET request
        if tx.send(cmd).await.is_err() {
            eprintln!("connection task shutdown");
            return;
        }

        // Await the response
        let res = resp_rx.await;
        println!("GOT = {:?}", res);
    });

    let t2 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Set {
            key: "foo".to_string(),
            val: b"bar".to_vec(),
            resp: resp_tx,
        };

        // Send the SET request
        if tx2.send(cmd).await.is_err() {
            eprintln!("connection task shutdown");
            return;
        }

        // Await the response
        let res = resp_rx.await;
        println!("GOT = {:?}", res)
    });

    // we `.await` the join handles to ensure the
    // commands fully complete before the process exits.
    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
}

// output
// GOT = Ok(Ok(None))
// GOT = Ok(Ok(()))
