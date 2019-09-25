use crate::communication::messages::{
    MessageType, RunCommandRequest, RunCommandResponse, MESSAGE_LENGTH_SIZE, MESSAGE_TYPE_SIZE,
};
use crate::os;
use crate::os::run_command;
use std::io::{Cursor, Error, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::str::from_utf8;
use std::thread;

use crate::communication::utils::get_msg_type_and_length;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use ron;

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

fn handle_message(data: &[u8], size: usize, msg_type: u8, mut stream: &TcpStream) {
    if msg_type == MessageType::RunCommandType as u8 {
        println!("Run command type!");
        let request: RunCommandRequest = ron::de::from_bytes(data).unwrap();
        let response = run_command_message(request).unwrap();
        let serialized_message = ron::ser::to_string(&response).unwrap();
        let message_len = serialized_message.len();
        let message_type = MessageType::RunCommandType as u8;

        let mut buffer: Vec<u8> =
            Vec::with_capacity(message_len + MESSAGE_TYPE_SIZE + MESSAGE_LENGTH_SIZE);
        buffer.push(message_type);
        buffer.write_u32::<BigEndian>(message_len as u32).unwrap();
        buffer.extend(serialized_message.into_bytes());

        println!("Buffer sending: {:?}", &buffer);
        stream.write(&buffer).unwrap();
    } else if msg_type == MessageType::DownloadFileType as u8 {
        println!("Download file type!")
    }
}

fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    println!("Handling connection from {}", stream.peer_addr()?);
    let mut type_and_length = [0 as u8; MESSAGE_TYPE_SIZE + MESSAGE_LENGTH_SIZE];
    while match stream.read(&mut type_and_length) {
        Ok(size) => match size {
            0 => false,
            _ => {
                let (msg_type, msg_length) = get_msg_type_and_length(type_and_length);

                let mut message = vec![0; msg_length];
                let size = stream
                    .read(&mut message)
                    .expect("Could not read message after getting message metadata. Error: {}");
                // read function guarantees that we will read all data
                handle_message(&message, msg_length, msg_type, &stream);
                true
            }
        },
        Err(e) => {
            println!(
                "An error occurred, terminating connection with {}. Error: {}",
                stream.peer_addr()?,
                e
            );
            stream.shutdown(Shutdown::Both)?;
            false
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
    panic!("Exit listener loop unexpectedly")
}
