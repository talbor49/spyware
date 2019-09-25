use std::io::Error;
use std::net::TcpStream;

pub fn connect_to_backdoor(addr: &str) -> Result<TcpStream, Error> {
    match TcpStream::connect(addr) {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 1337");

            Ok(stream)
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
            Err(e)
        }
    }
}
