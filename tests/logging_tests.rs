use spyware::logging::core::{get_logs, setup_logging, LoggingConfiguration};

use log;

#[test]
fn test_logging_sanity() {
    println!("Running a server in the background");
    // Setup logging
    unsafe {
        // This function is unsafe as it mutates the global logging state, initializing it.
        // We are calling it before using any logging functionality (which would've been pointless before initialization).
        // Also, we are calling it before creating any threads.
        // Therefore, this is a safe operation.
        setup_logging(LoggingConfiguration {
            to_stdout: true,
            to_memory: true,
            // Allow max 10,000 characters to be written to log memory
            // This is 4096 * 4 = 16kb.
            max_memory_log_size_bytes: 4096 * std::mem::size_of::<char>(),
            level: log::LevelFilter::Debug,
        });
    }
    assert_eq!(get_logs().len(), 0);
    log::debug!("Hello world!");
    assert_eq!(get_logs().len(), 1);
}
