use log::{Metadata, Record, LevelFilter};
use queues::{CircularBuffer, IsQueue};
use crate::logging::memory_logger::CircularMemoryLogs;
use std::borrow::{Borrow, BorrowMut};

struct MemoryLogger {
    conf: LoggingConfiguration
}

impl log::Log for MemoryLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.conf.to_memory
    }

    fn log(&self, record: &Record) {
        // TODO make this really safe
        unsafe {
            if CIRCULAR_MEMORY_LOGS.is_some() {
                CIRCULAR_MEMORY_LOGS.as_mut().unwrap().write_log(record.args().to_string());
            }
        }
    }

    fn flush(&self) {}
}

impl MemoryLogger {
    pub fn print_all_logs(&self) {
        unsafe {
            println!("{}", &CIRCULAR_MEMORY_LOGS.as_mut().unwrap().get_all_logs().join("\n"));
        }
    }
}

pub struct LoggingConfiguration {
    pub to_stdout: bool,
    pub to_memory: bool,
    pub max_memory_log_size_bytes: usize,
    pub level: log::LevelFilter
}

const DEFAULT_CONF: LoggingConfiguration = LoggingConfiguration {
    to_stdout: true,
    to_memory: true,
    max_memory_log_size_bytes: 1024,
    level: log::LevelFilter::Error
};

static mut MEMORY_LOGGER: MemoryLogger = MemoryLogger {
    conf: DEFAULT_CONF
};
static mut CIRCULAR_MEMORY_LOGS: Option<CircularMemoryLogs> = None;

// This functions in unsafe. It mutates the global logger state in memory.
// The caller must use it wisely.
// It should only be called once, while the program is initialized, before any log mutation might happen.
// It would be pointless to use any logging functionality before initializing it anyway.
pub unsafe fn setup_logging(configuration: LoggingConfiguration) {
    MEMORY_LOGGER.conf = configuration;
    CIRCULAR_MEMORY_LOGS = Some(CircularMemoryLogs::new());
    log::set_logger(&MEMORY_LOGGER);
    log::set_max_level(MEMORY_LOGGER.conf.level.clone());
}

pub fn get_logs() -> &'static Vec<String> {
    unsafe {
        CIRCULAR_MEMORY_LOGS.as_mut().unwrap().get_all_logs()
    }
}