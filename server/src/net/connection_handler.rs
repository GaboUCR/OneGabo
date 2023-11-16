use tokio::net::TcpStream;
use tokio_tungstenite::accept_async;
use futures_util::{StreamExt, SinkExt};
use tokio_tungstenite::tungstenite::Error;
use tungstenite::Message;

enum MsgCode {
    InitialMessage,
    CreateFile,
    NewFile,
    DeleteFile
}

pub async fn handle_connection(socket: TcpStream) {
    let mut ws_stream = accept_async(socket).await.expect("Error on the websocket handshake");

    while let Some(message) = ws_stream.next().await {
        match message {
            Ok(msg) => {
                if msg.is_binary() || msg.is_text() {
                    // Suponiendo que el primer byte es el código del mensaje
                    // println!("{}", msg.into_data().get(0).unwrap());
                    if let Some(code) = msg.into_data().get(0) {
                        match code {
                            0 => { ws_stream.send(Message::text("Respuesta 1234")).await.expect("Failed to send message");},
                            1 => { /* Lógica para CreateFile */ },
                            2 => { /* Lógica para NewFile */ },
                            3 => { /* Lógica para DeleteFile */ },
                            _ => eprintln!("Código de mensaje desconocido"),
                        }
                    }
                }

                // Aquí puedes enviar una respuesta si es necesario
                // ws_stream.send(Message::text("Respuesta")).await.expect("Failed to send message");
            }
            Err(e) => {
                eprintln!("Error processing WebSocket message: {:?}", e);
                break;
            }
        }
    }
}