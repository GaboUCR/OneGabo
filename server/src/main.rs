use tokio::task;
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Error;
use futures_util::{StreamExt, SinkExt};
#[tokio::main]
async fn main() {
    print!("Server listening!!");
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    while let Ok((socket, _)) = listener.accept().await {

        task::spawn(handle_connection(socket));   
    }
}

async fn handle_connection (socket:tokio::net::TcpStream) {
    let mut ws_stream = accept_async(socket).await.expect("Error on the websocket handshake");

    while let Some(message) = ws_stream.next().await {

        match message {

            Ok(msg) => {
                if msg.is_text() || msg.is_binary() {
                    // Process the message
                    // For example, echo the message back
                    ws_stream.send(msg).await.expect("Failed to send message");
                }
            }
            Err(e) => {
                eprintln!("Error processing WebSocket message: {:?}", e);
                break;
            }


        }
    }

}