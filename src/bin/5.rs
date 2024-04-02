use std::time::Instant;
use adventofcode2016::aoc_reader;
use md5;


fn main() {
    let content = aoc_reader(5);
    let start = Instant::now(); // skip file IO in timing
    let (result1, result2) = door_password(&content);

    let duration = start.elapsed();
    println!(
        "Part 1 result: {result1}\n\
         Part 2 result: {result2}"
    );
    println!("Execution took {:?}", duration)
}

fn door_password(data: &str) -> (String, String) {
    let mut index : i32 = 0;
    let mut password1 = vec![];
    let mut password2 = vec!['_';8];
    for _ in 0..8 {
        while password1.len() <= 8 || password2.contains(&'_') {
            let result = md5::compute(format!("{}{}",data, index));
            let result_string =format!("{:X}",result);
            if  result_string.starts_with("00000"){
                let first_char = result_string.chars().nth(5).unwrap();
                password1.push(first_char);
                let first_digit = first_char.to_digit(10).unwrap_or(9) as usize;
                if first_digit <= 7  && password2[first_digit] == '_' {
                    let second_char = result_string.chars().nth(6).unwrap();
                    password2[first_digit] = second_char;
                }
            }
            index += 1;
        }
    }
    password1.truncate(8);
    (password1.iter().collect::<String>().to_lowercase(), password2.iter().collect::<String>().to_lowercase())
}

