use std::io::{Error, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;
use log::{debug, info, error};

use ron;

use crate::communication::messages::{
    DownloadFileRequest, GetBasicInfoRequest, Message, MessageType, MessageTypes,
    RunCommandRequest, MESSAGE_HEADER_LENGTH,
};
use crate::communication::serialization::{extract_msg_type_and_length, serialize_message};
use serde::Serialize;

use crate::actions::basic_info::{download_file_message, get_basic_info_request};
use crate::actions::commands::run_command_message;
use num_traits::FromPrimitive;

pub const BIND_ANY: &str = "0.0.0.0";

fn send_response(
    response: impl Serialize + MessageType,
    mut stream: &TcpStream,
) -> Result<(), Error> {
    let response_buffer = serialize_message(response)?;
    debug!("Sending response buffer: {:?}", &response_buffer);
    stream.write_all(&response_buffer)?;
    Ok(())
}

fn handle_message(message: Message, stream: &TcpStream) {
    match MessageTypes::from_u8(message.message_type) {
        Some(MessageTypes::RunCommandRequest) => {
            let request: RunCommandRequest =
                ron::de::from_bytes(&message.serialized_message).unwrap();
            let response = run_command_message(request);
            send_response(response, stream).unwrap();
        }
        Some(MessageTypes::DownloadFileRequest) => {
            let request: DownloadFileRequest =
                ron::de::from_bytes(&message.serialized_message).unwrap();
            let response = download_file_message(request);
            send_response(response, stream).unwrap();
        }
        Some(MessageTypes::GetBasicInfoRequest) => {
            let _request: GetBasicInfoRequest =
                ron::de::from_bytes(&message.serialized_message).unwrap();
            let response = get_basic_info_request();
            send_response(response, stream).unwrap();
        }
        _ => error!("Unrecognized message type '{}'", message.get_type()),
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
            Ok(Message {
                message_type: msg_type,
                serialized_message_length: msg_length,
                serialized_message: message,
            })
        }
        Err(e) => {
            error!(
                "An error occurred, terminating connection with {}. Error: {}",
                stream.peer_addr()?,
                e
            );
            stream.shutdown(Shutdown::Both)?;
            Err(e)
        }
    }
}

pub fn handle_client(stream: TcpStream) -> Result<(), Error> {
    debug!("Handling connection from {}", stream.peer_addr()?);
    while match get_message(&stream) {
        Ok(message) => {
            handle_message(message, &stream);
            true
        }
        Err(e) => {
            error!(
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
            info!("Listening on: {:?}", address);
        }
        Err(e) => {
            error!("Error {} while trying to get local address.", &e);
        }
    };
    // accept connections and process them in a new thread
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                info!("New connection with: {}", stream.peer_addr()?);
                thread::spawn(move || {
                    // connection succeeded
                    // TODO what to do if this returns Error
                    handle_client(stream)
                });
            }
            Err(e) => {
                error!("Connection failed: {}", e);
                /* connection failed */
            }
        }
    }
    // TODO handle this panic
    panic!("Server closed unexpectedly")
}
