use serde::{Deserialize, Serialize};

pub enum MessageType {
    RunCommandType,
    DownloadFileType,
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
