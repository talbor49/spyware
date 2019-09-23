use std::io::{Error, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::process::{Command, Output};
use std::str::from_utf8;
use std::thread;

mod communication;
mod os;

fn main() {
    loop {
        match communication::run_server() {
            Ok(_) => (),
            Err(e) => {
                println!("Error {} when running server. Trying again.", e);
            }
        }
    }
}
