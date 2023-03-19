use std::io::Error;

//Uncomment this block to pass the first stage
use tokio::{net::{TcpListener, TcpStream}, io::{AsyncWriteExt, AsyncReadExt}};

async fn handle_connection(mut socket: TcpStream)-> Result<(), Error> {
    let mut buf = [0; 1024];

    // In a loop, read data from the socket and write the data back.
    loop {
        match socket.read(&mut buf).await {
            // socket closed
            Ok(n) if n == 0 => break,
            Ok(n) => socket.write_all(b"+PONG\r\n").await?,
            Err(e) => {
                println!("failed to read from socket; err = {:?}", e);
                break;
            }
        };
    }
    
    Ok(())
}
#[tokio::main]
async fn main() -> Result<(), Error>{
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    
    let listener = TcpListener::bind("127.0.0.1:6379").await?;
    
    loop {
        let stream = listener.accept().await;
        match stream {
            Ok((mut _stream, _)) => {
                println!("accepted new connection");
                tokio::spawn(async move {
                    match handle_connection(_stream).await {
                        Ok(_) => println!("Connection closed"),
                        Err(_) => println!("error "),
                    }
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
