use regex::Regex;

pub struct MemoryParser {
    mul_regex: Regex,
    do_regex: Regex,
    dont_regex: Regex,
}

impl MemoryParser {
    pub fn new() -> Self {
        Self {
            mul_regex: Regex::new(r"mul\((\d+),(\d+)\)").unwrap(),
            do_regex: Regex::new(r"do\(\)").unwrap(),
            dont_regex: Regex::new(r"don't\(\)").unwrap(),
        }
    }

    pub fn parse(&self, memory: &str) -> Vec<Instruction> {
        let mut instructions = Vec::new();
        let mut enabled = true;

        for token in memory.split(|c: char| !c.is_alphanumeric() && c != '(' && c != ')' && c != ',' && c != '\'') {
            if self.do_regex.is_match(token) {
                enabled = true;
            } else if self.dont_regex.is_match(token) {
                enabled = false;
            } else if let Some(caps) = self.mul_regex.captures(token) {
                let x: i32 = caps[1].parse().unwrap();
                let y: i32 = caps[2].parse().unwrap();
                instructions.push(Instruction::Mul { x, y, enabled });
            }
        }

        instructions
    }
}

#[derive(Debug)]
pub enum Instruction {
    Mul { x: i32, y: i32, enabled: bool },
}
