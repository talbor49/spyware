use crate::logging::core::get_logs;
use crate::communication::messages::{GetLogsResponse, ErrorInfo};
use failure::Fail;
use log::{debug, error};

pub fn get_logs_request() -> GetLogsResponse {
    debug!("Got get logs request!");
    match get_logs() {
        Ok(logs) => {
            GetLogsResponse {
                logs,
                error_info: None
            }
        },
        Err(err) => {
            error!("Could not get logs");
            GetLogsResponse {
                logs: Vec::new(),
                error_info: Some(ErrorInfo {
                    raw_os_error: 1,
                    as_string: err.name().unwrap().to_string()
                })
            }
        }
    }
}