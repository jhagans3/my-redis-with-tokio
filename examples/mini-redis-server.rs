use bytes::Bytes;
use mini_redis::{Connection, Frame};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;

// Instead of using Vec<u8> we use the bytes crate.
// The biggest feature it adds over Vec<u8> is shallow cloning.
// calling clone() on a Bytes instance
// does not copy the underlying data.
// Instead, a Bytes instance is a reference-counted handle
// to some underlying data. The Bytes type
// is roughly an Arc<Vec<u8>> but with some added capabilities.
type Db = Arc<Mutex<HashMap<String, Bytes>>>;

#[derive(Debug)]
enum Command {
    Get { key: String },
    Set { key: String, val: Bytes },
}

// cargo run --example mini-redis-server
#[tokio::main]
async fn main() {
    // Bind the listener to the address
    let mut listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    println!("Listening");

    // common error is to unconditionally use tokio::sync::Mutex
    // from within async code. An async mutex is a mutex that is
    // locked across calls to `.await`. A synchronous mutex will
    // block the current thread when waiting to acquire the lock.
    // This, in turn, will block other tasks from processing.
    // tokio::sync::Mutex usually uses a synchronous mutex internally.
    // As a rule of thumb, using a synchronous mutex from within
    // asynchronous code is fine as long as contention remains
    // low and the lock is not held across calls to `.await`.
    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        // The second item contains the ip and port of the new connection.
        let (socket, _) = listener.accept().await.unwrap();
        // Clone the handle
        let db = db.clone();

        println!("Accepted");
        process(socket, db).await;
    }
}

async fn process(socket: TcpStream, db: Db) {
    use mini_redis::Command::{self, Get, Set};

    // Connection, provided by `mini-redis`, handles parsing frames from
    // the socket
    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        // Write the response to the client
        connection.write_frame(&response).await.unwrap();
    }
}
