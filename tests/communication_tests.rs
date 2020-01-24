use std::io::{Error, Write};
use std::net::{Shutdown, TcpStream};
use std::thread;

use rand::Rng;
use rustdoor::communication;
use rustdoor::communication::messages::RunCommandRequest;
use rustdoor::communication::serialization::serialize_message;

const LOOPBACK_IP: &str = "127.0.0.1";
const MIN_PORT_VALUE: u16 = 1024;
const MAX_PORT_VALUE: u16 = 65535;

pub fn connect_to_backdoor(addr: &str) -> Result<TcpStream, Error> {
    match TcpStream::connect(addr) {
        Ok(stream) => {
            println!("Successfully connected to backdoor at address {}", addr);
            Ok(stream)
        }
        Err(e) => {
            println!("Failed to connect to backdoor: {}", e);
            Err(e)
        }
    }
}

fn run_server_and_connect(port: u16) -> Result<TcpStream, Error> {
    // Test will fail on panic if run_server or connect_to_backdoor fails
    thread::spawn(move || communication::server::run_server(port).unwrap());
    let hundred_millis = std::time::Duration::from_millis(100);
    thread::sleep(hundred_millis);
    let stream = connect_to_backdoor(&format!("{}:{}", LOOPBACK_IP, port))?;
    return Ok(stream);
}

#[test]
fn test_basic_connection() {
    let random_port = rand::thread_rng().gen_range(MIN_PORT_VALUE, MAX_PORT_VALUE);
    // Test will fail on panic, if run server or connect fails
    let stream = run_server_and_connect(random_port).unwrap();
    // This will print some errors like:
    // An error occurred, terminating connection with 127.0.0.1:2886. Error: failed to fill whole buffer.
    // This is because we are closing connection unexpectedly. It's ok.
    stream.shutdown(Shutdown::Both).unwrap();
}

#[test]
fn test_send_basic_command() {
    let random_port = rand::thread_rng().gen_range(MIN_PORT_VALUE, MAX_PORT_VALUE);
    let mut stream = run_server_and_connect(random_port).unwrap();
    let message = RunCommandRequest {
        command: String::from("dir"),
        async_run: false,
    };
    let buffer = serialize_message(message).unwrap();
    stream.write(&buffer).unwrap();
}
