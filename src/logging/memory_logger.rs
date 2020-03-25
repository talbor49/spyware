use std::sync::{Mutex};

pub struct CircularMemoryLogs {
    logs: Mutex<Vec<String>>,
    total_max_bytes: usize,
    current_bytes_count: usize,
}

impl CircularMemoryLogs {
    pub fn new(total_max_bytes: usize) -> Self {
        CircularMemoryLogs {
            logs: Mutex::new(Vec::<String>::new()),
            total_max_bytes,
            current_bytes_count: 0
        }
    }

    pub fn write_log(&mut self, log: String) {
        if log.len() >= self.total_max_bytes {
            // Log is too big to handle. This should not happen, but we should not panic
            return
        }
        self.current_bytes_count += log.len();
        let mut logs = self.logs.lock().unwrap();
        while self.current_bytes_count > self.total_max_bytes {
            self.current_bytes_count -= logs.get(0).unwrap().len();
            logs.remove(0);
        }
        logs.push(log);
    }

    pub fn get_all_logs(&mut self) -> &Vec<String> {
        // TODO is there a way to get an an *immutable* reference to the inner of value of mutex? So we don't need to do mutable borrowing here.
        self.logs.get_mut().unwrap()
    }
}
