use tokio::net::TcpStream;
use tokio_tungstenite::accept_async;
use futures_util::{StreamExt, SinkExt};
use tokio_tungstenite::tungstenite::Error;
use tungstenite::Message;
use chrono::{DateTime, Utc};
use crate::storage::file_manager::{save_bytes_to_file, create_folder, delete_file_or_directory};
use crate::network::ws_message_handler::{MsgCode, slice_to_int_le, bytes_to_string_lossy};
use crate::handlers::{handle_delete_file, handle_update_file};
use std::convert::TryInto;

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
    
    while let Some(message) = ws_stream.next().await {
        match message {
            Ok(msg) => {
                    if (msg.is_binary()) {
                        let mut msg_content = msg.into_data(); 
                        if let Some(code) = MsgCode::from_byte(&msg_content[..4]) {
                            match code {
                                MsgCode::InitialMessage => {
                                    ws_stream.send(Message::text("Respuesta 1234")).await.expect("Failed to send message");
                                },
                                MsgCode::CreateFile => {},
                                MsgCode::NewFile => {},
                                MsgCode::DeleteFile => {handle_delete_file(msg_content, &client_ip).await;},
                                MsgCode::UpdateFile => {handle_update_file(msg_content, &client_ip).await;}
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