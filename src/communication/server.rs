use std::io::{Error, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

use ron;

use crate::communication::messages::{
    Message, MessageType, MessageTypes, RunCommandRequest, RunCommandResponse,
    MESSAGE_HEADER_LENGTH,
};
use crate::communication::serialization::{extract_msg_type_and_length, serialize_message};
use crate::os;

pub const BIND_ANY: &str = "0.0.0.0";

fn run_command_message(request: RunCommandRequest) -> Result<RunCommandResponse, Error> {
    let result = os::run_command(&request.command);
    match result {
        Ok(output) => {
            println!("Command execution succeed, output: {}", output);
            return Ok(RunCommandResponse {
                output,
                error_code: 0,
            });
        }
        Err(e) => {
            println!("Command execution failed, error: {}", e);
            return Ok(RunCommandResponse {
                output: String::from(""),
                error_code: e.raw_os_error().unwrap_or(0),
            });
        }
    }
}

fn handle_message(message: Message, mut stream: &TcpStream) {
    if message.get_type() == MessageTypes::RunCommandRequest as u8 {
        println!("Run command type!");
        let request: RunCommandRequest = ron::de::from_bytes(&message.serialized_message).unwrap();
        // TODO handle malformed messages instead of panicking
        let response = run_command_message(request).unwrap();
        let buffer = serialize_message(response).unwrap();
        println!("Buffer sending: {:?}", &buffer);
        stream.write(&buffer).unwrap();
    } else {
    }
}

pub fn get_message(mut stream: &TcpStream) -> Result<Message, Error> {
    let mut type_and_length = [0 as u8; MESSAGE_HEADER_LENGTH];
    match stream.read_exact(&mut type_and_length) {
        Ok(()) => {
            let (msg_type, msg_length) = extract_msg_type_and_length(type_and_length);
            let mut message = vec![0; msg_length];

            // Read_exact function guarantees that we will read exactly enough data to fill the buffer
            stream
                .read_exact(&mut message)
                .expect("Could not read message after getting message metadata. Error: {}");
            return Ok(Message {
                message_type: msg_type,
                serialized_message_length: msg_length,
                serialized_message: message,
            });
        }
        Err(e) => {
            println!(
                "An error occurred, terminating connection with {}. Error: {}",
                stream.peer_addr()?,
                e
            );
            stream.shutdown(Shutdown::Both)?;
            return Err(e);
        }
    }
}

fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    println!("Handling connection from {}", stream.peer_addr()?);
    while match get_message(&mut stream) {
        Ok(message) => {
            handle_message(message, &stream);
            true
        }
        Err(e) => {
            println!(
                "An error occurred while trying to get message. Error: {}",
                e
            );
            return Err(e);
        }
    } {}
    Ok(())
}

pub fn run_server(port: u16) -> Result<(), Error> {
    // If we can't open the server, it probably means:
    // - The port is already taken, or we are not running in sufficient permissions
    // - If we are not running in sufficient permissions, panic.
    // - If the port is open, also panic (In the future might add a feature to handle it)
    let listener = TcpListener::bind(format!("{}:{}", BIND_ANY, port))?;
    match listener.local_addr() {
        Ok(address) => {
            println!("Listening on: {:?}", address);
        }
        Err(e) => {
            println!("Error {} while trying to get local address.", &e);
        }
    };
    // accept connections and process them in a new thread
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection with: {}", stream.peer_addr()?);
                thread::spawn(move || {
                    // connection succeeded
                    // TODO what to do if this returns Error
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Connection failed: {}", e);
                /* connection failed */
            }
        }
    }
    // TODO catch this panic
    panic!("Server closed unexpectedly")
}
