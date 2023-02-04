use std::collections::HashMap;

pub struct School {
    grades: HashMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            grades: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.grades
            .entry(grade)
            .or_default()
            .push(String::from(student));
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut ret: Vec<u32> = self.grades.keys().copied().collect();
        ret.sort();
        ret
    }

    pub fn grade(&self, grade: u32) -> Vec<String> {
        let mut ret: Vec<String> = self.grades.get(&grade).cloned().unwrap_or_default();
        ret.sort();
        ret
    }
}

impl Default for School {
    fn default() -> Self {
        Self::new()
    }
}
