use tokio::sync::mpsc;

// cargo run --example muti-send
#[tokio::main]
async fn main() {
    // When every `Sender` has gone out of scope or has otherwise been dropped,
    // it is no longer possible to send more messages into the channel.
    // At this point, the `recv` call on the Receiver will return None,
    // which means that all senders are gone and the channel is closed.
    let (mut tx, mut rx) = mpsc::channel(32);
    let mut tx2 = tx.clone();

    tokio::spawn(async move {
        let _result = tx.send("sending from first handle").await;
    });

    tokio::spawn(async move {
        let _result = tx2.send("sending from second handle").await;
    });

    // Both messages are sent to the
    // single Receiver handle. It is
    // not possible to clone the
    // receiver of an mpsc channel.
    while let Some(message) = rx.recv().await {
        println!("GOT = {}", message);
    }
}

// output
// GOT = sending from second handle
// GOT = sending from first handle
