use std::io::Error;
use std::net::{Shutdown, TcpStream};
use std::thread;

use rustdoor::communication;
use rustdoor::communication::server::{LOOPBACK_IP, PORT};

pub fn connect_to_backdoor(addr: &str) -> Result<TcpStream, Error> {
    match TcpStream::connect(addr) {
        Ok(stream) => {
            println!("Successfully connected to backdoor in port 1337");
            Ok(stream)
        }
        Err(e) => {
            println!("Failed to connect to backdoor: {}", e);
            Err(e)
        }
    }
}
#[test]
fn test_basic_connection() {
    // Test will fail on panic, if run server or connect fails
    let stream = run_server_and_connect();
    // This will print some errors like:
    // An error occurred, terminating connection with 127.0.0.1:2886. Error: failed to fill whole buffer.
    // This is because we are closing connection unexpectedly. It's ok.
    stream.shutdown(Shutdown::Both).unwrap();
}

fn run_server_and_connect() -> TcpStream {
    // Test will fail on panic, if run server or connect fails
    thread::spawn(|| communication::server::run_server(PORT).unwrap());
    let hundred_millis = std::time::Duration::from_millis(100);
    thread::sleep(hundred_millis);
    let stream = connect_to_backdoor(&format!("{}:{}", LOOPBACK_IP, PORT)).unwrap();
    return stream;
}
