#![allow(dead_code)]
use std::fs;

mod day1;
mod day2;
mod day3;

pub fn get_input(filename: &str) -> String {
    fs::read_to_string(format!("input/{filename}"))
        .expect(&format!("Missing input/{filename}"))
        .trim()
        .to_string()
}
