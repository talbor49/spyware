use spyware::logging::core::{destroy_logging, get_logs, setup_logging, LoggingConfiguration};

/// We are using one big test here instead of small tests, because logging can only be initialized once per instance of the program.
/// This covers all our needed tests here instead in one instance of the program.
#[test]
fn test_logging() {
    destroy_logging();
    // Should not panic
    log::info!("Hello world!");
    // Should be error
    assert!(get_logs().is_err());

    setup_logging(LoggingConfiguration {
        to_stdout: true,
        to_memory: true,
        max_memory_log_size_bytes: 48 * std::mem::size_of::<char>(),
        level: log::LevelFilter::Info,
    })
    .unwrap();
    assert_eq!(get_logs().unwrap().len(), 0);

    // Logging level is info, so debug should not be logged
    log::debug!("A");
    assert_eq!(get_logs().unwrap().len(), 0);

    // This log is too big to store - so it should be ignored
    log::info!("Hey this log is bigger than 48 chars so it won't be stored at all! Also as this is an error in logging mechanism it won't be logged :O");
    assert_eq!(get_logs().unwrap().len(), 0);

    log::info!("A");
    assert_eq!(get_logs().unwrap().len(), 1);
    log::info!("B");
    assert_eq!(get_logs().unwrap().len(), 2);
    // This will rotate the logs, removing logs until there is enough space to hold this log.
    log::info!("AAAAAAAAAAA");
    assert_eq!(get_logs().unwrap().len(), 1);
}
