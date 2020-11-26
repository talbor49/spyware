use crate::communication::messages::{ErrorInfo, RunCommandRequest, RunCommandResponse};
use cmd_lib::run_fun;
use log::{debug, error};
use std::io::Error;

pub fn run_command(command: &str) -> Result<String, Error> {
    run_fun!("{}", command)
}

pub fn run_command_message(request: RunCommandRequest) -> RunCommandResponse {
    debug!(
        "Got run command request: run command \"{}\" !",
        &request.command
    );
    let result = run_command(&request.command);
    match result {
        Ok(output) => {
            debug!("Command execution succeed, output: {}", output);
            RunCommandResponse {
                output,
                error_info: None,
            }
        }
        Err(err) => {
            error!("Command execution failed, error: {}", err);
            RunCommandResponse {
                output: String::from(""),
                error_info: Some(ErrorInfo {
                    raw_os_error: err.raw_os_error().unwrap_or(-1),
                    as_string: err.to_string(),
                }),
            }
        }
    }
}
