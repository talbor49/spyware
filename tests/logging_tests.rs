use spyware::logging::core::{get_logs, setup_logging, LoggingConfiguration, destroy_logging};

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

fn clear_logging_state() {
    unsafe {
        destroy_logging()
    }
}

#[test]
fn test_logging_sanity() {
    clear_logging_state();

    for _ in 0..1000 {
        do_setup_logging(LoggingConfiguration {
            to_stdout: true,
            to_memory: true,
            // Allow max 10,000 characters to be written to log memory
            // This is 4096 * 4 = 16kb.
            max_memory_log_size_bytes: 4096 * std::mem::size_of::<char>(),
            level: log::LevelFilter::Debug,
        });

        println!("Actually, look at the fucking logs: {:#?}", get_logs().unwrap());
        for _ in 0..10000 {
            assert_eq!(get_logs().unwrap().len(), 0);
        }
        // log::debug!("Hello world!");
        // assert_eq!(get_logs().unwrap().len(), 1);

        clear_logging_state();
    }
}

#[test]
fn test_logging_levels() {
    clear_logging_state();

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
    clear_logging_state();

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
    clear_logging_state();

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
    clear_logging_state();

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

#[test]
fn test_logging_rotation() {
    for _ in 0..1000 {
        clear_logging_state();
        do_setup_logging(LoggingConfiguration {
            to_stdout: true,
            to_memory: true,
            max_memory_log_size_bytes: 48 * std::mem::size_of::<char>(),
            level: log::LevelFilter::Info
        });
        // 10 chars are allowed
        log::info!("A");
        assert_eq!(get_logs().unwrap().len(), 1);
        log::info!("B");
        assert_eq!(get_logs().unwrap().len(), 2);
        log::info!("AAAAAAAA");
        assert_eq!(get_logs().unwrap().len(), 1);
    }
}
