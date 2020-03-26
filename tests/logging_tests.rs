use spyware::logging::core::{get_logs, setup_logging, LoggingConfiguration};

use log;

fn do_setup_logging(conf: LoggingConfiguration) {
    // Setup logging
    unsafe {
        // This function is unsafe as it mutates the global logging state, initializing it.
        // We are calling it before using any logging functionality (which would've been pointless before initialization).
        // Also, we are calling it before creating any threads.
        // Therefore, this is a safe operation.
        setup_logging(conf);
    }
}

#[test]
fn test_logging_sanity() {
    println!("Running a server in the background");

    do_setup_logging(LoggingConfiguration {
        to_stdout: true,
        to_memory: true,
        // Allow max 10,000 characters to be written to log memory
        // This is 4096 * 4 = 16kb.
        max_memory_log_size_bytes: 4096 * std::mem::size_of::<char>(),
        level: log::LevelFilter::Debug,
    });

    assert_eq!(get_logs().unwrap().len(), 0);
    log::debug!("Hello world!");
    assert_eq!(get_logs().unwrap().len(), 1);
}

#[test]
fn test_logging_levels() {
    do_setup_logging(LoggingConfiguration {
        to_stdout: true,
        to_memory: true,
        max_memory_log_size_bytes: 4096,
        level: log::LevelFilter::Error
    });
    assert_eq!(get_logs().unwrap().len(), 0);
    log::debug!("Hello world!");
    log::info!("Hello world!");
    assert_eq!(get_logs().unwrap().len(), 0);
    log::error!("Hello world!");
    assert_eq!(get_logs().unwrap().len(), 1);
}

#[test]
fn test_logging_disable_memory_logging() {
    do_setup_logging(LoggingConfiguration {
        to_stdout: false,
        to_memory: false,
        max_memory_log_size_bytes: 4096,
        level: log::LevelFilter::Debug
    });
    assert!(get_logs().is_err());
    // Should not panic
    log::info!("Hello world!");
    assert!(get_logs().is_err());
}

#[test]
fn test_logging_log_too_big_to_store() {
    do_setup_logging(LoggingConfiguration {
        to_stdout: true,
        to_memory: true,
        max_memory_log_size_bytes: 4,
        level: log::LevelFilter::Info
    });
    assert_eq!(get_logs().unwrap().len(), 0);
    log::info!("Hey this log is bigger than 4 bytes so it won't be stored at all.");
    assert_eq!(get_logs().unwrap().len(), 0);
}

#[test]
fn test_logging_no_setup() {
    // Should not panic
    log::info!("Hello world!");
}

#[test]
fn test_logging_late_setup() {
    // Should not panic
    log::info!("Hello world!");
    assert!(get_logs().is_err());
    do_setup_logging(LoggingConfiguration {
        to_stdout: true,
        to_memory: true,
        max_memory_log_size_bytes: 4096,
        level: log::LevelFilter::Info
    });
    assert_eq!(get_logs().unwrap().len(), 0);
    log::info!("Hello, World!");
    assert_eq!(get_logs().unwrap().len(), 1);
}
