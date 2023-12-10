//server/src/network/ws_message_handler.rs
use tokio::net::TcpStream;
use tokio_tungstenite::accept_async;
use futures_util::{StreamExt, SinkExt};
use tokio_tungstenite::tungstenite::Error;
use tungstenite::Message;
use chrono::{DateTime, Utc};
use std::convert::TryInto;

pub enum MsgCode {
    InitialMessage = 0,
    CreateFile = 1,
    UpdateFile = 2,
    NewFile = 3,
    DeleteFile = 4
}

impl MsgCode {
    pub fn from_byte(byte: &[u8]) -> Option<Self> {

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

pub fn slice_to_int_le(slice: &[u8]) -> i32 {
    let (int_bytes, _) = slice.split_at(std::mem::size_of::<i32>());
    i32::from_le_bytes(int_bytes.try_into().expect("slice with incorrect length"))
}

pub fn bytes_to_string_lossy(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}