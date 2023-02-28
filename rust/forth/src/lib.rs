use std::{collections::HashMap, result::Result};

pub type Value = i32;

#[derive(Default)]
pub struct Forth {
    stack: Vec<Value>,
    custom_definitions: Vec<Vec<String>>,
    custom_words: HashMap<String, usize>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

fn is_number(s: &str) -> bool {
    s.parse::<Value>().is_ok()
}

impl Forth {
    pub fn new() -> Forth {
        Forth::default()
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    fn peek(&mut self) -> Result<Value, Error> {
        if let Some(&v) = self.stack.last() {
            Ok(v)
        } else {
            Err(Error::StackUnderflow)
        }
    }

    fn pop(&mut self) -> Result<Value, Error> {
        if let Some(v) = self.stack.pop() {
            Ok(v)
        } else {
            Err(Error::StackUnderflow)
        }
    }

    fn eval_parts(&mut self, parts: &[String]) -> Result<(), Error> {
        let mut parts = parts.iter();
        while let Some(part) = parts.next() {
            if let Ok(n) = part.parse::<i32>() {
                self.stack.push(n);
            } else if let Some(&index) = self.custom_words.get(part) {
                let definition = &self.custom_definitions[index];
                self.eval_parts(&definition.clone())?;
            } else if let Some(index) = part.strip_prefix("__internal__") {
                let index: usize = index.parse().unwrap();
                let definition = &self.custom_definitions[index];
                self.eval_parts(&definition.clone())?;
            } else if part == "+" {
                let b = self.pop()?;
                let a = self.pop()?;
                self.stack.push(a + b);
            } else if part == "-" {
                let b = self.pop()?;
                let a = self.pop()?;
                self.stack.push(a - b);
            } else if part == "*" {
                let b = self.pop()?;
                let a = self.pop()?;
                self.stack.push(a * b);
            } else if part == "/" {
                let b = self.pop()?;
                let a = self.pop()?;
                self.stack
                    .push(a.checked_div(b).ok_or(Error::DivisionByZero)?);
            } else if part == "dup" {
                let a = self.peek()?;
                self.stack.push(a);
            } else if part == "drop" {
                self.pop()?;
            } else if part == "swap" {
                let a = self.pop()?;
                let b = self.pop()?;
                self.stack.push(a);
                self.stack.push(b);
            } else if part == "over" {
                let b = self.pop()?;
                let a = self.pop()?;
                self.stack.push(a);
                self.stack.push(b);
                self.stack.push(a);
            } else if part == ":" {
                let &word = &parts.next().ok_or(Error::InvalidWord)?;
                if is_number(word) {
                    return Err(Error::InvalidWord);
                }
                let mut definition = Vec::new();
                loop {
                    let word = parts.next().ok_or(Error::InvalidWord)?;
                    if word == ";" {
                        break;
                    }
                    if let Some(index) = self.custom_words.get(word) {
                        definition.push(format!("__internal__{}", index));
                    } else {
                        definition.push(word.clone());
                    }
                }
                let index = self.custom_definitions.len();
                self.custom_definitions.push(definition);
                self.custom_words.insert(String::from(word), index);
            } else {
                return Err(Error::UnknownWord);
            }
        }
        Ok(())
    }

    pub fn eval(&mut self, input: &str) -> Result<(), Error> {
        let input = input.to_lowercase();
        let parts: Vec<String> = input.split_whitespace().map(String::from).collect();
        self.eval_parts(&parts)
    }
}
