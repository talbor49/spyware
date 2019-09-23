use std::{thread, time};
use std::net::{TcpListener, TcpStream, Shutdown};
use std::time::Duration;
use std::io::{Read, Write};

const CHUNKS: usize = 1024;
const ONE_SEC: Duration = time::Duration::from_secs(1);
const BIND_ADDR: &str = "0.0.0.0:1337";

fn handle_client(mut stream: TcpStream) {
    println!("Handling connection from {}", stream.peer_addr().unwrap());
    let mut data = [0 as u8; CHUNKS];
    while match stream.read(&mut data) {
        Ok(size) => {
            // echo everything!
            stream.write(&data[0..size]).unwrap();
            println!("Received {:?}", &data[0..size]);
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
