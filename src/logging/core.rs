use log::{Metadata, Record};

use crate::logging::memory_logger::CircularMemoryLogs;

use failure::{Fail};

#[derive(Debug, Fail)]
pub enum LoggingError {
    #[fail(display = "Logging was not initialized, try calling setup_logging")]
    LoggingNotInitializedError
}


struct MemoryLogger {}

impl log::Log for MemoryLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        // This is safe - write_log function is using mutex and is thread safe.
        unsafe {
            if CIRCULAR_MEMORY_LOGS.is_some() {
                CIRCULAR_MEMORY_LOGS
                    .as_mut()
                    .unwrap()
                    .write_log(format!("{} {}: {}", record.level(), record.target(), record.args()));
            }
        }
    }

    fn flush(&self) {}
}

impl MemoryLogger {
    pub fn print_all_logs(&self) {
        unsafe {
            println!(
                "{}",
                &CIRCULAR_MEMORY_LOGS
                    .as_mut()
                    .unwrap()
                    .get_all_logs()
                    .join("\n")
            );
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

static MEMORY_LOGGER: MemoryLogger = MemoryLogger {};
static mut CIRCULAR_MEMORY_LOGS: Option<CircularMemoryLogs> = None;

unsafe fn setup_memory_logging(max_memory_log_size_bytes: usize) {
    CIRCULAR_MEMORY_LOGS = Some(CircularMemoryLogs::new(max_memory_log_size_bytes));
    log::set_logger(&MEMORY_LOGGER);
}

// This functions in unsafe. It mutates the global logger state in memory.
// The caller must use it wisely.
// It should only be called once, while the program is initialized, before any log mutation might happen.
// It would be pointless to use any logging functionality before initializing it anyway.
pub unsafe fn setup_logging(configuration: LoggingConfiguration) {
    log::set_max_level(configuration.level.clone());
    if configuration.to_memory {
        setup_memory_logging(configuration.max_memory_log_size_bytes);
    }
}

pub fn get_logs() -> Result<&'static Vec<String>, LoggingError> {
    unsafe {
        match CIRCULAR_MEMORY_LOGS.as_mut() {
            Some(logs) => Ok(logs.get_all_logs()),
            None => Err(LoggingError::LoggingNotInitializedError)
        }
    }
}
