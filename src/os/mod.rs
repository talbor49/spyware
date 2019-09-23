use std::io::Error;
use std::process::{Command, Output};

pub fn run_command(comm: &str) -> Result<Output, Error> {
    Command::new(comm).output()
}
