use crate::communication::messages::{ErrorInfo, RunCommandRequest, RunCommandResponse};
use cmd_lib::run_fun;
use std::io::Error;

pub fn run_command(command: &str) -> Result<String, Error> {
    println!("Running command '{}'", &command);
    run_fun!("{}", command)
}

pub fn run_command_message(request: RunCommandRequest) -> RunCommandResponse {
    let result = run_command(&request.command);
    match result {
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
    }
}
