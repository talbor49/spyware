use std::io::Error;
use std::process::{Command, Output};

pub fn run_command(comm: &str) -> Result<Output, Error> {
    println!("Running command '{}'", &comm);
    Command::new(comm).output()
}
