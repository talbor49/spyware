mod command_tests;
use command_tests::run_server_and_connect;
use rand::Rng;
use std::io::{Error, Write};

use rustdoor::communication::messages::{DownloadFileRequest, RunCommandRequest};
use rustdoor::communication::serialization::serialize_message;
use std::net::{Shutdown, TcpStream};

const MIN_PORT_VALUE: u16 = 1024;
const MAX_PORT_VALUE: u16 = 65535;

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
