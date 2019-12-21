use cmd_lib::run_fun;
use std::io::Error;

pub fn run_command(command: &str) -> Result<String, Error> {
    println!("Running command '{}'", &command);
    run_fun!("{}", command)
}
