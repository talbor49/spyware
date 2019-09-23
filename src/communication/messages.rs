use serde::{Deserialize, Serialize};

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
