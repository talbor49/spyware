
pub struct CircularMemoryLogs {
    logs: Vec<String>,
    current_bytes_count: usize
}

impl CircularMemoryLogs {
    pub fn new() -> Self {
        CircularMemoryLogs {
            logs: Vec::<String>::new(),
            current_bytes_count: 0
        }
    }

    pub fn write_log(&mut self, log: String) {
        println!("Writing log!");
        self.logs.push(log);
    }

    pub fn get_all_logs(&self) -> &Vec< String> {
        &self.logs
    }
}
