use std::{thread, time};

use rustdoor::communication::server::PORT;

mod communication;
pub mod os;

const RETRY_INTERVAL_SECONDS: u64 = 60;

fn main() {
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
