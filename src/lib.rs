pub mod point;

use std::fs;


pub fn aoc_reader(day: i8) -> String{
    let filename = format!("inputs/day{day}.txt");
    let content = fs::read_to_string(filename).expect("Error reading file {filename}");
    return content;
}



