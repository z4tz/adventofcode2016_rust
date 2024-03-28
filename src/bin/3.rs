use std::time::Instant;
use adventofcode2016::aoc_reader;
use itertools::Itertools;

fn main() {
    let content = aoc_reader(3);
    let start = Instant::now(); // skip file IO in timing
    let result1 = count_triangles(&content);
    let result2 = count_triangles_alternative(&content);
    let duration = start.elapsed();
    println!(
        "Part 1 result: {result1}\n\
         Part 2 result: {result2}"
    );
    println!("Execution took {:?}", duration)
}

fn count_triangles(data: &str) -> usize {
    data.lines().map(to_numbers).filter(valid_triangle).count()
}

fn count_triangles_alternative(data: &str) -> usize {
    let mut total: usize = 0;
    for lines in &data.lines().chunks(3) {
        // convert 3 lines to numbers and rotate them (first row becomes first column etc.)
        let mut container = vec![vec![]; 3];
        for line in lines {
            for (i, number) in to_numbers(line).iter().enumerate() {
                container[i].push(*number)
            }
        }
        for number_vec in container {
            if valid_triangle(&number_vec) {
                total += 1;
            }
        }
    }
    total
}

fn to_numbers(line: &str) -> Vec<i32> {
    let mut numbers = line.trim().split_whitespace()
        .map(|a| a.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    numbers.sort();
    numbers
}

fn valid_triangle(numbers: &Vec<i32>) -> bool {
    numbers[0] + numbers[1] > numbers[2]
}

