use tokio::net::TcpStream;
use tokio_tungstenite::accept_async;
use futures_util::{StreamExt, SinkExt};
use tokio_tungstenite::tungstenite::Error;
use tungstenite::Message;
use chrono::{DateTime, Utc};
use crate::storage::file_manager::{save_bytes_to_file, create_folder};
use crate::network::ws_message_handler::{MsgCode, slice_to_int_le, bytes_to_string_lossy};
use std::convert::TryInto;

async fn handle_update_file (msg_content:Vec<u8>, ip_address: &str) {

    let filename_size = slice_to_int_le(&msg_content[4..8]) as usize;
    let filename_end_index = filename_size + 8;

    let mut file_name = bytes_to_string_lossy(&msg_content[8..filename_end_index]);
    let data = &msg_content[filename_end_index..];

    let root = format!("./drive/{}/", ip_address);

    file_name.insert_str(0, &root);
    println!("file name:{} \n file content: {}", file_name, bytes_to_string_lossy(data));
    save_bytes_to_file(data, &file_name).await;
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
                                MsgCode::DeleteFile => {},
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