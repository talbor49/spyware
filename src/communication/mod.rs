use std::io::{Cursor, Error, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::str::from_utf8;
use std::thread;

use byteorder::{BigEndian, ReadBytesExt};
use ron;

use crate::communication::messages::{Message, RunCommandResponse};
use crate::os;

mod messages;

const CHUNKS: usize = 1024;
const MESSAGE_TYPE_SIZE: usize = 1;
const MESSAGE_LENGTH_SIZE: usize = 4;
const BIND_ADDR: &str = "0.0.0.0:1337";

fn handle_message_too(msg: Message) {}

fn handle_message(data: &[u8], size: usize, msg_type: u8, mut stream: &TcpStream) {
    // echo everything!
    println!("Received {:?}", &data[0..size]);
    let output = os::run_command(from_utf8(&data[0..size]).unwrap());
    let output = output.unwrap();

    let response = RunCommandResponse {
        stdout: output.stdout,
        stderr: output.stderr,
        error_code: output.status.code().unwrap_or(-1),
    };

    //            println!("Output: {}", from_utf8(&output.stdout).unwrap());
    println!("Message: {:?}", response);

    let serialized_message = ron::ser::to_string(&response).unwrap();

    println!("Message serialized : {:?}", &serialized_message);
    stream.write(serialized_message.as_bytes()).unwrap();
}

fn get_msg_type_and_length(
    type_and_length: [u8; MESSAGE_TYPE_SIZE + MESSAGE_LENGTH_SIZE],
) -> (u8, usize) {
    let msg_type = type_and_length[0];
    let msg_length = &type_and_length[MESSAGE_TYPE_SIZE..MESSAGE_TYPE_SIZE + MESSAGE_LENGTH_SIZE];
    let mut rdr = Cursor::new(msg_length);
    let msg_length = rdr.read_u32::<BigEndian>().unwrap() as usize;
    return (msg_type, msg_length);
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
        Err(E) => {
            println!(
                "An error occurred, terminating connection with {}. Error: {}",
                stream.peer_addr()?,
                E
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
