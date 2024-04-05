use std::time::Instant;
use adventofcode2016::aoc_reader;
use fancy_regex::Regex;

fn main() {
    let content = aoc_reader(7);
    let start = Instant::now(); // skip file IO in timing
    let (result1, result2) = protocol_support(&content);

    let duration = start.elapsed();
    println!(
        "Part 1 result: {result1}\n\
         Part 2 result: {result2}"
    );
    println!("Execution took {:?}", duration)
}


fn protocol_support(data: &str) -> (i32, i32){
    let tls_regex = Regex::new(r"(.)(?!\1)(.)\2\1").unwrap();
    let mut valid_tls = 0;
    let mut valid_ssl = 0;
    for line in data.lines() {
        if tls_valid(line, &tls_regex) {
            valid_tls += 1
        }
        if ssl_valid(line){
            valid_ssl += 1
        }
    }
    (valid_tls, valid_ssl)
}

fn tls_valid(string: &str, re: &Regex) -> bool {
    let parts = string.split(&['[', ']']).collect();
    let (outside, inside) = separate_parts(parts);
    let abba_found = outside.iter()
        .map(|a| re.is_match(a).unwrap())
        .any(|a| a == true);
    let no_hypernet_abba = inside.iter()
        .map(|a| re.is_match(a).unwrap())
        .all(|a|a==false);

    return abba_found && no_hypernet_abba
}

fn ssl_valid(string: &str) -> bool {
    let parts = string.split(&['[', ']']).collect();
    let (supernet, hypernet ) = separate_parts(parts);

    let babs: Vec<String> = supernet.iter()
        .map(get_babs)
        .flatten()
        .collect();

    for value in hypernet {
        for bab in &babs {
            if value.contains(bab) {
                return true
            }
        }
    }
    false
}

fn get_babs(string: &&str) -> Vec<String> {
    let mut babs: Vec<String> = vec![];
    let chars: Vec<char> = string.chars().collect();
    for i in 0..chars.len()-2{
        if chars[i] == chars[i+2] && chars[i] != chars[i+1] {
            babs.push(vec![chars[i+1], chars[i], chars[i+1]].iter().collect())
        }
    }
    babs
}

fn separate_parts(strings: Vec<&str>) -> (Vec<&str>, Vec<&str>) {
    let mut vec1 = vec![];
    let mut vec2 = vec![];
    for (index, value) in strings.iter().enumerate() {
        match index%2 {
            0 => vec1.push(*value),
            1 => vec2.push(*value),
            _ => println!("This should not happen")
        }
    }
    (vec1, vec2)
}