use tokio::net::TcpStream;
use tokio_tungstenite::accept_async;
use futures_util::{StreamExt, SinkExt};
use tokio_tungstenite::tungstenite::Error;
use tungstenite::Message;

enum MsgCode {
    InitialMessage = 0,
    CreateFile = 1,
    UpdateFile = 2,
    NewFile = 3,
    DeleteFile = 4
}

impl MsgCode {
    fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            0 => Some(MsgCode::InitialMessage),
            1 => Some(MsgCode::CreateFile),
            2 => Some(MsgCode::UpdateFile),
            3 => Some(MsgCode::NewFile),
            4 => Some(MsgCode::DeleteFile),
            _ => None,
        }
    }
}

pub async fn handle_connection(socket: TcpStream) {
    let mut ws_stream = accept_async(socket).await.expect("Error on the websocket handshake");

    while let Some(message) = ws_stream.next().await {
        match message {
            Ok(msg) => {
                if let Some(code_byte) = msg.into_data().get(0) {
                    if let Some(code) = MsgCode::from_byte(*code_byte) {
                        match code {
                            MsgCode::InitialMessage => {
                                ws_stream.send(Message::text("Respuesta 1234")).await.expect("Failed to send message");
                            },
                            MsgCode::CreateFile => {},
                            MsgCode::NewFile => {},
                            MsgCode::DeleteFile => {},
                            MsgCode::UpdateFile => {}
                            // ... otros casos ...
                        }
                    } else {
                        eprintln!("Código de mensaje desconocido");
                    }
                }
            }
            Err(e) => {
                eprintln!("Error processing WebSocket message: {:?}", e);
                break;
            }
        }
    }
}