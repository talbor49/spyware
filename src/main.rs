mod communication;
mod os;

fn main() {
    loop {
        match communication::server::run_server() {
            Ok(_) => (),
            Err(e) => {
                println!("Error {} when running server. Trying again.", e);
            }
        }
    }
}
