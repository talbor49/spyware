mod communication;
mod os;

fn main() {
    loop {
        match communication::run_server() {
            Ok(_) => (),
            Err(e) => {
                println!("Error {} when running server. Trying again.", e);
            }
        }
    }
}
