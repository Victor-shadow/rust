use async_std::{task, net::TcpListener, net::TcpStream};
use futures::AsyncWriteExt;

async fn process_request(mut stream: TcpStream) -> Result<(), std::io::Error>{
    stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n").await?;
    stream.write_all(b"Hello World").await?;
    Ok(())
}


fn main() {
    task::block_on(async {
        let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

        println!("Server running on http://127.0.0.1:8080");

        while let Ok((stream, _addr)) = listener.accept().await {
            task::spawn(async move {
                if let Err(e) = process_request(stream).await {
                    eprintln!("Failed to process request: {}", e);
                }
            });
        }
    });
}
