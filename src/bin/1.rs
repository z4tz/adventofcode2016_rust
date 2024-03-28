use num::complex::Complex;
use std::collections::HashSet;
use std::time::Instant;
use adventofcode2016::aoc_reader;

fn main() {
    let content = aoc_reader(1);
    let start = Instant::now(); // skip file IO in timing
    let (result1, result2) = hq_distance(&content);
    let duration = start.elapsed();
    println!(
        "Part 1 result: {result1}\n\
         Part 2 result: {result2}"
    );
    println!("Execution took {:?}", duration)
}

struct Walker {
    location: Complex<i32>,
    direction: Complex<i32>,
}

impl Walker {
    fn new() -> Self {
        // Walker starts at 0,0 facing north
        Walker {
            location: Complex::new(0, 0),
            direction: Complex::new(0, 1),
        }
    }
}

const LEFT_TURN: Complex<i32> = Complex::new(0, 1);
const RIGHT_TURN: Complex<i32> = Complex::new(0, -1);

fn hq_distance(data: &str) -> (i32, i32) {
    let mut walker = Walker::new();
    let mut visited: HashSet<Complex<i32>> = HashSet::new();
    let mut first_double_visit = Complex::new(0, 0);
    let mut double_visit_found = false;
    for part in data.split(", ") {
        let mut part_chars = part.chars();
        let turn = part_chars.next().unwrap();
        let steps = part_chars.as_str().parse::<i32>().unwrap();
        match turn {
            'L' => walker.direction *= LEFT_TURN,
            'R' => walker.direction *= RIGHT_TURN,
            _ => (),
        }

        for _ in 0..steps {
            walker.location += walker.direction;
            if !double_visit_found {
                if visited.contains(&walker.location) {
                    first_double_visit = walker.location.clone();
                    double_visit_found = true;
                }
                visited.insert(walker.location);
            }
        }
    }
    (distance_from_origo(walker.location), distance_from_origo(first_double_visit))
}

fn distance_from_origo(location: Complex<i32>) -> i32 {
    location.re.abs() + location.im.abs()
}