use log::{Metadata, Record};
use std::sync::Mutex;

use crate::logging::memory_logger::CircularMemoryLogs;

use failure::{Fail};

#[derive(Debug, Fail)]
pub enum LoggingError {
    #[fail(display = "Logging was not initialized, try calling setup_logging")]
    LoggingNotInitializedError,
    #[fail(display = "setup_logging function was called twice, although logging can be initialized once.")]
    LoggingSetupCalledTwice
}


struct MemoryLogger {
    inner_logger: Option<std::sync::Mutex<CircularMemoryLogs>>
}

impl log::Log for MemoryLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        match self.inner_logger.as_ref() {
            Some(inner_logger_mutex) => {
                inner_logger_mutex.lock().unwrap().write_log(
                    format!("{} {}: {}", record.level(), record.target(), record.args())
                )
            },
            None => {}
        }
    }

    fn flush(&self) {}
}

impl MemoryLogger {
    fn init(&mut self, total_max_bytes: usize) -> Result<(), LoggingError> {
        match self.inner_logger {
            Some(_) => Err(LoggingError::LoggingSetupCalledTwice),
            None => {
                self.inner_logger = Some(
                    std::sync::Mutex::new(
                        CircularMemoryLogs::new(total_max_bytes)
                    )
                );
                Ok(())
            }
        }
    }

    fn destroy(&mut self) {
        self.inner_logger = None
    }

    fn get_logs(&mut self) -> Result<&Vec<String>, LoggingError>{
        match self.inner_logger.as_mut() {
            Some(inner_logger) => {
                Ok(inner_logger.get_mut().unwrap().get_all_logs())
            },
            None => Err(LoggingError::LoggingNotInitializedError)
        }
    }
}

pub struct LoggingConfiguration {
    pub to_stdout: bool,
    pub to_memory: bool,
    pub max_memory_log_size_bytes: usize,
    pub level: log::LevelFilter,
}

const DEFAULT_CONF: LoggingConfiguration = LoggingConfiguration {
    to_stdout: true,
    to_memory: true,
    max_memory_log_size_bytes: 1024,
    level: log::LevelFilter::Error,
};

static mut MEMORY_LOGGER: MemoryLogger = MemoryLogger {
    inner_logger: None,
};

// This functions in unsafe. It mutates the global logger state in memory.
// The caller must use it wisely.
// It should only be called once, while the program is initialized, before any log mutation might happen.
// It would be pointless to use any logging functionality before initializing it anyway.
pub unsafe fn setup_logging(configuration: LoggingConfiguration) -> Result<(), LoggingError> {
    if configuration.to_memory {
        MEMORY_LOGGER.init(configuration.max_memory_log_size_bytes)?;
        log::set_logger(&MEMORY_LOGGER);
    }
    log::set_max_level(configuration.level.clone());
    Ok(())
}

pub unsafe fn destroy_logging() {
    MEMORY_LOGGER.destroy()
}

pub fn get_logs() -> Result<&'static Vec<String>, LoggingError> {
    unsafe {
        MEMORY_LOGGER.get_logs()
    }
}
