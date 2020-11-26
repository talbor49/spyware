use std::sync::RwLock;

pub struct CircularMemoryLogs {
    logs: RwLock<Vec<String>>,
    total_max_chars: usize,
}

fn compute_used_chars(logs: &[String]) -> usize {
    logs.iter().map(|log| log.len()).sum()
}

impl CircularMemoryLogs {
    pub fn new(total_max_bytes: usize) -> Self {
        // Convert bytes to chars here so use is more convenient in the rest of the class
        CircularMemoryLogs {
            logs: RwLock::new(Vec::<String>::new()),
            total_max_chars: total_max_bytes / std::mem::size_of::<char>(),
        }
    }

    pub fn write_log(&mut self, log: String) {
        if log.len() >= self.total_max_chars {
            // Log is too big to save.
            return;
        }
        let mut logs = self.logs.write().unwrap();
        logs.push(log);
        while compute_used_chars(&logs) >= self.total_max_chars {
            logs.remove(0);
        }
    }

    pub fn get_all_logs(&self) -> Vec<String> {
        self.logs.read().unwrap().clone()
    }

    pub fn clear_all_logs(&self) {
        self.logs.write().unwrap().clear()
    }
}
