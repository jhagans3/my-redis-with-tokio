use mini_redis::{client, Result};

// start mini redis server
// mini-redis-server
// $ cargo run --bin hello-mini-redis
#[tokio::main]
pub async fn main() -> Result<()> {
    // Open a connection to the mini-redis address.
    // Asynchronously establishes a TCP connection
    // with the specified remote address.
    // Once the connection is established
    // a client handle is returned.
    let mut client = client::connect("127.0.0.1:6379").await?;

    // Set the key "hello" with value "world"
    client.set("hello", "world".into()).await?;

    // Get key "hello"
    let result = client.get("hello").await?;

    println!("got value from the server; result={:?}", result);

    Ok(())
}
