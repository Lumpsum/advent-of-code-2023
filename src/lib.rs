mod day;
pub mod template;
pub mod repository;

use std::str::Lines;

pub use day::*;


pub fn get_lines(input: &str) -> Lines {
    return input.lines()
}
