use crate::logging::core::get_logs;
use crate::communication::messages::{GetLogsResponse, ErrorInfo};
use failure::Fail;

pub fn get_logs_request() -> GetLogsResponse {
    debug!("Got get logs request!");
    let l: Vec<String> = Vec::new();
    match get_logs() {
        Ok(logs) => {
            GetLogsResponse {
                logs,
                error_info: None
            }
        },
        Err(err) =>
            GetLogsResponse {
                logs: vec::new(),
                error_info: Some(ErrorInfo {
                    raw_os_error: err.into(),
                    as_string: err.name().unwrap().into_string()
                })
        }
    }
}