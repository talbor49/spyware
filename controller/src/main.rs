use ron;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

use rustdoor::communication::messages::{MessageType, RunCommandRequest};

fn make_run_command_request(command: String, async_run: bool) {
    let req = RunCommandRequest { command, async_run };
    let serialized_message = ron::ser::to_string(&req).unwrap();
    let message_type = MessageType::RunCommandType as u8;
    let message_len = serialized_message.len();
}

pub fn open_door() {
    match TcpStream::connect("localhost:1337") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 1337");

            let msg = b"Hello!";

            stream.write(msg).unwrap();
            println!("Sent Hello, awaiting reply...");

            let mut data = [0 as u8; 6]; // using 6 byte buffer
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    if &data == msg {
                        println!("Reply is ok!");
                    } else {
                        let text = from_utf8(&data).unwrap();
                        println!("Unexpected reply: {}", text);
                    }
                }
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}

fn main() {
    open_door();
    println!("Terminated.");
}
