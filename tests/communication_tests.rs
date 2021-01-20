pub mod communication_utils;
use communication_utils::run_server_and_connect;
use std::io::{Read, Write};

use spyware::communication::messages::RunCommandRequest;
use spyware::communication::serialization::serialize_message;
use std::net::Shutdown;

#[test]
fn test_basic_connection() {
    // Test will fail on panic, if run server or connect fails
    let stream = run_server_and_connect().unwrap();
    // This will print some errors like:
    // An error occurred, terminating connection with 127.0.0.1:2886. Error: failed to fill whole buffer.
    // This is because we are closing connection unexpectedly. It's ok.
    stream.shutdown(Shutdown::Both).unwrap();
}

#[test]
fn test_send_basic_command() {
    const TO_ECHO: &str = "alksmdlkasmdklamsdklamksldmklamsd";
    let mut stream = run_server_and_connect().unwrap();
    let message = RunCommandRequest {
        command: String::from(format!("echo {}", TO_ECHO)),
        async_run: false,
    };
    let buffer = serialize_message(
        spyware::communication::messages::Message::RunCommandRequest { 0: message },
    )
    .unwrap();
    stream
        .write(&buffer)
        .expect("Could not write to server after connection");
    let mut response_buffer = Vec::new();
    stream
        .read(&mut response_buffer)
        .expect("Could not read response from server");
}
