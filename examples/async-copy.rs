use tokio::fs::File;
use tokio::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut reader: &[u8] = b"hello";
    let mut file = File::create("foo.txt").await?;

    // asynchronously copies the entire contents of a reader into a writer
    io::copy(&mut reader, &mut file).await?;
    Ok(())
}
