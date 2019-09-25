use serde::{Deserialize, Serialize};

pub enum MessageType {
    RunCommandType(RunCommandRequest),
    DownloadFileType(DownloadFileRequest),
}

pub enum MessageToo {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
    RunCommandRequest,
}

pub struct Message {
    pub message_type: MessageType,
    pub message_length: usize,
    pub message: MessageToo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RunCommandRequest {
    command: String,
    async_run: bool,
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
