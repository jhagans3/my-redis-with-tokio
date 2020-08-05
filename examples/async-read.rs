use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};

// ls ../data/foo.txt
// cargo run --example async-read
#[tokio::main]
async fn main() -> io::Result<()> {
    let mut f = File::open("../data/foo.txt").await?;
    let mut buffer = [0; 10];

    // sync method for reading data into a buffer,
    // returning the number of bytes read
    // read up to 10 bytes
    let n = f.read(&mut buffer[..]).await?;

    println!("The bytes: {:?}", &buffer[..n]);

    Ok(())
}
