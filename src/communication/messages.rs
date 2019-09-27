use serde::{Deserialize, Serialize};

pub const MESSAGE_TYPE_SIZE: usize = 1;
pub const MESSAGE_LENGTH_SIZE: usize = 4;
pub const MESSAGE_HEADER_LENGTH: usize = MESSAGE_TYPE_SIZE + MESSAGE_LENGTH_SIZE;

pub enum MessageType {
    RunCommandType,
    DownloadFileType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub message_type: u8,
    pub serialized_message_length: usize,
    pub serialized_message: Vec<u8>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RunCommandRequest {
    pub command: String,
    pub async_run: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RunCommandResponse {
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    pub error_code: i32,
}

pub struct DownloadFileRequest {
    // ...
}
