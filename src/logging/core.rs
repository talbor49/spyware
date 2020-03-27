use log::{Metadata, Record};

use crate::logging::memory_logger::CircularMemoryLogs;

use failure::Fail;
use once_cell::sync::OnceCell;
use std::sync::RwLock;

#[derive(Debug, Fail)]
pub enum LoggingError {
    #[fail(display = "Logging was not initialized, try calling setup_logging")]
    LoggingNotInitializedError,
    #[fail(
        display = "setup_logging function was called twice, although logging can be initialized once."
    )]
    LoggingSetupCalledTwice,
    #[fail(display = "Could not initialize logging.")]
    LoggingInitializationError,
}

struct MemoryLogger {
    inner_logger: RwLock<CircularMemoryLogs>,
}

impl log::Log for MemoryLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        self.inner_logger.write().unwrap().write_log(format!(
            "{} {}: {}",
            record.level(),
            record.target(),
            record.args()
        ));
    }

    fn flush(&self) {}
}

impl MemoryLogger {
    fn global() -> Option<&'static MemoryLogger> {
        MEMORY_LOGGER_INSTANCE.get()
    }

    fn get_logs(&self) -> Result<Vec<String>, LoggingError> {
        Ok(self.inner_logger.read().unwrap().get_all_logs())
    }

    fn clear(&self) {
        self.inner_logger.write().unwrap().clear_all_logs()
    }
}

pub struct LoggingConfiguration {
    pub to_stdout: bool,
    pub to_memory: bool,
    pub max_memory_log_size_bytes: usize,
    pub level: log::LevelFilter,
}

static MEMORY_LOGGER_INSTANCE: OnceCell<MemoryLogger> = OnceCell::new();

// It should only be called once, while the program is initialized, before any log mutation might happen.
// It would be pointless to use any logging functionality before initializing it anyway.
pub fn setup_logging(configuration: LoggingConfiguration) -> Result<(), LoggingError> {
    if configuration.to_memory {
        let memory_logger = MemoryLogger {
            inner_logger: std::sync::RwLock::new(CircularMemoryLogs::new(
                configuration.max_memory_log_size_bytes,
            )),
        };
        match MEMORY_LOGGER_INSTANCE.set(memory_logger) {
            Ok(_) => log::set_logger(MemoryLogger::global().unwrap()).unwrap(),
            Err(_) => return Err(LoggingError::LoggingInitializationError)
        };
    }
    log::set_max_level(configuration.level);
    Ok(())
}

pub fn destroy_logging() {
    if let Some(logger) = MemoryLogger::global() {
        logger.clear()
    }
}

pub fn get_logs() -> Result<Vec<String>, LoggingError> {
    match MemoryLogger::global() {
        Some(logger) => Ok(logger.get_logs().unwrap()),
        _ => Err(LoggingError::LoggingNotInitializedError),
    }
}
