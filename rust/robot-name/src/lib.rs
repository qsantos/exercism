use std::sync::atomic::{AtomicUsize, Ordering};

const DEFAULT_NAME: String = String::new();

static ALLOCATED_NAMES: AtomicUsize = AtomicUsize::new(0);

pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        let mut ret = Robot { name: DEFAULT_NAME };
        ret.reset_name();
        ret
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        let mut number = ALLOCATED_NAMES.fetch_add(1, Ordering::SeqCst);
        let mut name = String::new();
        for _ in 0..2 {
            name.push((b'A' + (number % 26) as u8) as char);
            number /= 26;
        }
        for _ in 0..3 {
            name.push((b'0' + (number % 10) as u8) as char);
            number /= 10;
        }
        self.name = name;
    }
}

impl Default for Robot {
    fn default() -> Self {
        Self::new()
    }
}
