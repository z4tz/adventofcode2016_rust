use std::time::Instant;
use adventofcode2016::aoc_reader;
use std::collections::HashMap;
use std::iter::zip;

fn main() {
    let content = aoc_reader(6);
    let start = Instant::now(); // skip file IO in timing
    let (result1, result2) = repeated_message(&content);
    let duration = start.elapsed();
    println!(
        "Part 1 result: {result1}\n\
         Part 2 result: {result2}"
    );
    println!("Execution took {:?}", duration)
}

fn repeated_message(data: &str) -> (String, String) {

    let mut counters: Vec<HashMap<char,i32>> = vec![HashMap::new();data.lines().next().unwrap().len()];
    for line in data.lines() {
        for (counter, char) in zip(counters.iter_mut(), line.chars()) {
            *counter.entry(char).or_insert(0) += 1;
        }
    }
    let mut most_common_chars = vec![];
    let mut least_common_chars = vec![];
    for counter in counters {
        let mut count_vec: Vec<(&char,&i32)> = counter.iter().collect();
        count_vec.sort_by(|a,b| a.1.cmp(b.1).reverse());
        most_common_chars.push(count_vec.first().unwrap().0.clone());
        least_common_chars.push(count_vec.last().unwrap().0.clone());
    }
    (most_common_chars.iter().collect(), least_common_chars.iter().collect())
}
