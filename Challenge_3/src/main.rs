extern crate regex;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn find_mul(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut sum = 0;

    for cap in re.captures_iter(input) {
        let num1: i32 = cap[1].parse().unwrap();
        let num2: i32 = cap[2].parse().unwrap();
        sum += num1 * num2;
    }

    sum
}

fn remove_pattern(input: &str) -> String {
    let re = Regex::new(r"don't\(\).*?do\(\)").unwrap();
    re.replace_all(input, "").to_string()
}

fn process_string(input: &str) -> i32 {
    let cleaned_string = remove_pattern(input);
    println!("Cleaned string: {}", cleaned_string);
    find_mul(cleaned_string.as_str())
}

fn process_file(filename: &str) -> io::Result<i32> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut total_sum = 0;

    for line in reader.lines() {
        let line = line?;
        let mul_sum = process_string(&line);
        total_sum += mul_sum;
    }

    Ok(total_sum)
}

fn process_input(input: &str) -> i32 {
    let mut total_sum = 0;
    for line in input.lines() {
        let mul_sum = process_string(&line);
        total_sum += mul_sum;
    }
    total_sum
}

fn main() -> io::Result<()> {
    let input = include_str!("../input.txt");
    let total_sum = process_input(input);
    
    println!("Total sum of mul() results: {}", total_sum);
    Ok(())
}