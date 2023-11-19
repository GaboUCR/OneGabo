use tokio::net::TcpStream;
use tokio_tungstenite::accept_async;
use futures_util::{StreamExt, SinkExt};
use tokio_tungstenite::tungstenite::Error;
use tungstenite::Message;
use chrono::{DateTime, Utc};
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

struct File {
    path : String,
    date : DateTime<Utc>
}

pub async fn handle_connection(socket: TcpStream) {
    // Obtener la dirección IP del cliente
    let client_ip = match socket.peer_addr() {
        Ok(addr) => addr.ip().to_string(),
        Err(e) => {
            eprintln!("Error al obtener la dirección IP del cliente: {:?}", e);
            return;
        }
    };
    // Se realiza el handshake
    let mut ws_stream = accept_async(socket).await.expect("Error on the websocket handshake");
    // en este vector se guardan Structs de tipo file con los archivos que el usuario ha subido
    let mut files: Vec<File> = Vec::new(); 

    while let Some(message) = ws_stream.next().await {
        match message {
            Ok(msg) => {
                    if (msg.is_binary()) {
                        let mut msg_content = msg.into_data(); 
                        println!("Mensaje recibido: {:?}", msg_content);
                        if let Some(code) = MsgCode::from_byte(msg_content.remove(0)) {
                            match code {
                                MsgCode::InitialMessage => {
                                    ws_stream.send(Message::text("Respuesta 1234")).await.expect("Failed to send message");
                                },
                                MsgCode::CreateFile => {},
                                MsgCode::NewFile => {},
                                MsgCode::DeleteFile => {},
                                MsgCode::UpdateFile => {println!("{:?}", msg_content);}
                                // ... otros casos ...
                            }
                        } else {
                            eprintln!("Código de mensaje desconocido");
                        }
                    }
                    else {
                        continue;
                    }
                    }
                Err(e) => {
                    eprintln!("Error processing WebSocket message: {:?}", e);
                    break;
                }
        }
    }
}