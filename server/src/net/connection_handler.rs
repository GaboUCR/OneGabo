use tokio::net::TcpStream;
use tokio_tungstenite::accept_async;
use futures_util::{StreamExt, SinkExt};
use tokio_tungstenite::tungstenite::Error;

pub async fn handle_connection (socket:tokio::net::TcpStream) {
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