use std::sync::{Mutex};
use std::ops::SubAssign;

pub struct CircularMemoryLogs {
    logs: Mutex<Vec<String>>,
    total_max_chars: usize,
    current_chars_count: usize,
}

impl CircularMemoryLogs {
    pub fn new(total_max_bytes: usize) -> Self {
        // Convert bytes to chars here so use is more convenient in the rest of the class
        CircularMemoryLogs {
            logs: Mutex::new(Vec::<String>::new()),
            total_max_chars: total_max_bytes / std::mem::size_of::<char>(),
            current_chars_count: 0
        }
    }

    pub fn write_log(&mut self, log: String) {
        if log.len() >= self.total_max_chars {
            // Log is too big to save.
            return
        }
        self.current_chars_count += log.len();
        let mut logs = self.logs.lock().unwrap();
        while self.current_chars_count > self.total_max_chars {
            self.current_chars_count.sub_assign(
                logs.remove(0).len()
            );
        }
        logs.push(log);
    }

    pub fn get_all_logs(&mut self) -> &Vec<String> {
        // TODO is there a way to get an an *immutable* reference to the inner of value of mutex? So we don't need to do mutable borrowing here.
        self.logs.get_mut().unwrap()
    }
}
