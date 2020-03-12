use serde::{Deserialize, Serialize};

pub const MESSAGE_TYPE_SIZE: usize = 1;
pub const MESSAGE_LENGTH_SIZE: usize = 4;
pub const MESSAGE_HEADER_LENGTH: usize = MESSAGE_TYPE_SIZE + MESSAGE_LENGTH_SIZE;

pub enum MessageTypes {
    RunCommandRequest,
    RunCommandResponse,
    DownloadFileRequest,
    DownloadFileResponse,
}

pub trait MessageType {
    fn get_type(&self) -> u8;
}

#[derive(Debug)]
pub struct Message {
    pub message_type: u8,
    pub serialized_message_length: usize,
    pub serialized_message: Vec<u8>,
}

impl MessageType for Message {
    fn get_type(&self) -> u8 {
        self.message_type
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RunCommandRequest {
    pub command: String,
    pub async_run: bool,
}

impl MessageType for RunCommandRequest {
    fn get_type(&self) -> u8 {
        MessageTypes::RunCommandRequest as u8
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RunCommandResponse {
    pub output: String,
    pub error_code: i32,
}

impl MessageType for RunCommandResponse {
    fn get_type(&self) -> u8 {
        MessageTypes::RunCommandResponse as u8
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DownloadFileRequest {
    pub path: String
}

impl MessageType for DownloadFileRequest {
    fn get_type(&self) -> u8 {
        MessageTypes::DownloadFileRequest as u8
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DownloadFileResponse {
    pub file_data: Vec<u8>,
}

impl MessageType for DownloadFileResponse {
    fn get_type(&self) -> u8 {
        MessageTypes::DownloadFileResponse as u8
    }
}
