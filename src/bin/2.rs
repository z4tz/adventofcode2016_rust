use std::time::Instant;
use num::range;
use adventofcode2016::aoc_reader;
use adventofcode2016::point::Point;


fn main() {
    let content = aoc_reader(2);
    let start = Instant::now(); // skip file IO in timing
    let result1 = bathroom_code(&content, pads(1).unwrap());
    let result2 = bathroom_code(&content, pads(2).unwrap());
    let duration = start.elapsed();
    println!(
        "Part 1 result: {result1}\n\
         Part 2 result: {result2}"
    );
    println!("Execution took {:?}", duration)
}

fn bathroom_code(data: &str, pad: [[i32; 7]; 7]) -> String {
    let mut position = start_point(pad, 5).unwrap();
    let mut code: Vec<i32> = Vec::new();
    for line in data.lines() {
        for chr in line.trim().chars() {
            let temp_position = position + direction(chr);
            if pad[temp_position.y as usize][temp_position.x as usize] != 0 {
                position = temp_position;
            }
        }
        code.push(pad[position.y as usize][position.x as usize])
    }
    concat(&code)
}

fn direction(character: char) -> Point {
    match character {
        'U' => Point{x: 0, y: -1},
        'D' => Point{x: 0, y: 1},
        'L' => Point{x: -1, y: 0},
        'R' => Point{x: 1, y: 0},
        _ => Point{x: 0, y: 0}
    }
}

fn concat(vec: &[i32]) -> String {
    vec.iter().map(numpad_convert).collect::<String>()
}

fn numpad_convert(value: &i32) -> char {
    match value {
        10 => 'A',
        11 => 'B',
        12 => 'C',
        13 => 'D',
        _ => char::from_digit(value.clone() as u32, 10).unwrap()
    }
}

fn pads(choice: i8) -> Option<[[i32; 7];7]> {
    let a =  [[0; 7],
                        [0; 7],
                        [0,0,1,2,3,0,0],
                        [0,0,4,5,6,0,0],
                        [0,0,7,8,9,0,0],
                        [0; 7],
                        [0; 7]];
    let b =  [[0; 7],
                        [0,0,0,1,0,0,0],
                        [0,0,2,3,4,0,0],
                        [0,5,6,7,8,9,0],
                        [0,0,10,11,12,0,0],
                        [0,0,0,13,0,0,0],
                        [0; 7]];
    match choice {
        1 => Some(a),
        2 => Some(b),
        _ => None
    }
}

fn start_point(pad: [[i32; 7];7], start_value: i32) -> Option<Point> {
    for y in range(0, pad.len()) {
        for x in range(0, pad[0].len()) {
            if pad[y][x] == start_value {
                return Some(Point{x: x as i32, y: y as i32})
            }
        }
    }
    None
}