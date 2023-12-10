//server/src/handlers/handlers.rs
use futures_util::{StreamExt, SinkExt};
use tungstenite::Message;
use chrono::{DateTime, Utc};
use crate::storage::file_manager::{save_bytes_to_file, create_folder, delete_file_or_directory};
use crate::network::ws_message_handler::{MsgCode, slice_to_int_le, bytes_to_string_lossy};

pub async fn handle_update_file (msg_content:Vec<u8>, ip_address: &str) {

    let filename_size = slice_to_int_le(&msg_content[4..8]) as usize;
    let filename_end_index = filename_size + 8;

    let mut file_path = bytes_to_string_lossy(&msg_content[8..filename_end_index]);
    let data = &msg_content[filename_end_index..];

    let root = format!("./drive/{}/", ip_address);

    file_path.insert_str(0, &root);
    // println!("file name:{} \n file content: {}", file_path, bytes_to_string_lossy(data));
    save_bytes_to_file(data, &file_path).await;
}

pub async fn handle_delete_file (msg_content:Vec<u8>, ip_address: &str) {

    let mut file_path = bytes_to_string_lossy(&msg_content[4..]);
    let root = format!("./drive/{}/", ip_address);
    file_path.insert_str(0, &root);

    delete_file_or_directory(&file_path).await;
}