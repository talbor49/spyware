use crate::communication::messages::{
    DownloadFileRequest, DownloadFileResponse, ErrorInfo, GetBasicInfoResponse, RunCommandRequest,
    RunCommandResponse,
};
use crate::os;
use std::fs::read_to_string;

pub fn get_basic_info_request() -> GetBasicInfoResponse {
    GetBasicInfoResponse {
        version: "placeholder".to_string(),
        arch: "placeholder".to_string(),
        error_info: None,
    }
}

pub fn run_command_message(request: RunCommandRequest) -> RunCommandResponse {
    let result = os::run_command(&request.command);
    return match result {
        Ok(output) => {
            println!("Command execution succeed, output: {}", output);
            RunCommandResponse {
                output,
                error_info: None,
            }
        }
        Err(err) => {
            println!("Command execution failed, error: {}", err);
            RunCommandResponse {
                output: String::from(""),
                error_info: Some(ErrorInfo {
                    raw_os_error: err.raw_os_error().unwrap_or(-1),
                    as_string: err.to_string(),
                }),
            }
        }
    };
}

pub fn download_file_message(request: DownloadFileRequest) -> DownloadFileResponse {
    return match read_to_string(request.path) {
        Ok(data) => DownloadFileResponse {
            file_data: data.as_bytes().to_vec(),
            error_info: None,
        },
        Err(err) => DownloadFileResponse {
            file_data: vec![],
            error_info: Some(ErrorInfo {
                raw_os_error: err.raw_os_error().unwrap_or(-1),
                as_string: err.to_string(),
            }),
        },
    };
}
