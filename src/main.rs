use std::{thread, time};

mod communication;
pub mod os;

const RETRY_INTERVAL_SECONDS: u64 = 60;
const PORT: u32 = 13337;

fn run_server_loop() {
    loop {
        match communication::server::run_server(PORT) {
            Ok(_) => (),
            Err(e) => {
                println!(
                    "Error {} when running server. Trying again in {} seconds.",
                    e, RETRY_INTERVAL_SECONDS
                );
            }
        }
        let minute = time::Duration::from_secs(RETRY_INTERVAL_SECONDS);
        thread::sleep(minute);
    }
}

fn run_cnc_connection_loop() {
    loop {
        // For now, just sleep. In the future implement CNC
        std::thread::sleep(time::Duration::from_secs(RETRY_INTERVAL_SECONDS))
    }
}

fn main() {
    let server_handler = thread::spawn(|| run_server_loop());
    let cnc_connect = thread::spawn(|| run_cnc_connection_loop());

    cnc_connect
        .join()
        .expect("The cnc connection has panicked.");
    server_handler
        .join()
        .expect("The server connection has panicked.")
}
