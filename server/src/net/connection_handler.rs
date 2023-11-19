use tokio::net::TcpStream;
use tokio_tungstenite::accept_async;
use futures_util::{StreamExt, SinkExt};
use tokio_tungstenite::tungstenite::Error;
use tungstenite::Message;
use chrono::{DateTime, Utc};
use crate::storage::file_manager::save_bytes_to_file;
use std::convert::TryInto;
enum MsgCode {
    InitialMessage = 0,
    CreateFile = 1,
    UpdateFile = 2,
    NewFile = 3,
    DeleteFile = 4
}

impl MsgCode {
    fn from_byte(byte: &[u8]) -> Option<Self> {

        let code = slice_to_int_le(byte);
        match code {
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

fn slice_to_int_le(slice: &[u8]) -> i32 {
    let (int_bytes, _) = slice.split_at(std::mem::size_of::<i32>());
    i32::from_le_bytes(int_bytes.try_into().expect("slice with incorrect length"))
}

fn bytes_to_string_lossy(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

async fn handle_update_file (msg_content:Vec<u8>) {

    let filename_size = slice_to_int_le(&msg_content[4..8]) as usize;
    let filename_end_index = filename_size + 8;

    let file_name = bytes_to_string_lossy(&msg_content[8..filename_end_index]);
    let data = &msg_content[filename_end_index..];

    println!("file name:{} \n file content: {}", file_name, bytes_to_string_lossy(data));
    // save_bytes_to_file(data, file_path);
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
                        if let Some(code) = MsgCode::from_byte(&msg_content[..4]) {
                            match code {
                                MsgCode::InitialMessage => {
                                    ws_stream.send(Message::text("Respuesta 1234")).await.expect("Failed to send message");
                                },
                                MsgCode::CreateFile => {},
                                MsgCode::NewFile => {},
                                MsgCode::DeleteFile => {},
                                MsgCode::UpdateFile => {handle_update_file(msg_content).await;}
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