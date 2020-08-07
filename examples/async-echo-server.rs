use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut listener = TcpListener::bind("127.0.0.1:6142").await.unwrap();

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            // Copy data here

            // This fails to compile
            // this utility function takes a reader and a writer
            // and copies data from one to the other. However,
            // we only have a single TcpStream. This single value
            // implements both AsyncRead and AsyncWrite. Because
            // io::copy requires &mut for both the reader and the
            // writer, the socket cannot be used for both arguments
            // io::copy(&mut socket, &mut socket).await

            // A stack buffer is explicitly avoided, all task data that lives
            // across calls to .await must be stored by the task. In this case,
            // buf is used across `.await` calls. All task data is stored in a
            // single allocation. You can think of it as an enum where each
            // variant is the data that needs to be stored for a specific call to `.await`
            let mut buf = vec![0; 1024];

            loop {
                match socket.read(&mut buf).await {
                    // Return value of `Ok(0)` signifies that the remote has
                    // closed
                    Ok(0) => return,
                    Ok(n) => {
                        // Copy the data back to socket
                        if socket.write_all(&buf[..n]).await.is_err() {
                            // Unexpected socket error. There isn't much we can
                            // do here so just stop processing.
                            return;
                        }
                    }
                    Err(_) => {
                        // Unexpected socket error. There isn't much we can do
                        // here so just stop processing.
                        return;
                    }
                }
            }
        });
    }
}
