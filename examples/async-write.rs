use tokio::fs::File;
use tokio::io::{self, AsyncWriteExt};

// ls ../data/foo.txt
// my-redis-with-tokio/examples$ cargo run --example async-write
#[tokio::main]
async fn main() -> io::Result<()> {
    let mut file = File::create("../data/foo-write.txt").await?;

    // Writes some prefix of the byte string, but not necessarily all of it.
    // writes a buffer into the writer, returning how many bytes were written
    let n = file.write(b"some bytes").await?;

    println!("Wrote the first {} bytes of 'some bytes'.", n);
    Ok(())
}
