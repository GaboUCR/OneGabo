use tokio::task;
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Error;
use futures_util::{StreamExt, SinkExt};
mod net;
#[tokio::main]
async fn main() {
    println!("Server listening!!");
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    while let Ok((socket, _)) = listener.accept().await {

        task::spawn(net::handle_connection(socket));   
    }
}