use std::time::Instant;
use itertools::{enumerate};
use adventofcode2016::aoc_reader;

fn main() {
    let content = aoc_reader(8);
    let start = Instant::now(); // skip file IO in timing
    let result1 = code_display(&content);

    let duration = start.elapsed();
    println!(
        "Part 1 result: {result1}\n\
         Part 2 result: {result1}"
    );
    println!("Execution took {:?}", duration)
}

fn code_display(data: &str) -> u8 {
    let mut display = Display::new(50,6);
    for line in data.lines() {
        let parts: Vec<_> = line.split_whitespace().collect();

        // Create rect
        if parts[0] == "rect" {
            let values: Vec<usize> = parts[1].split('x')
                .collect::<Vec<&str>>()
                .iter()
                .map(|a|a.parse().unwrap())
                .collect();
            for y in 0..values[1] {
                for x in 0..values[0] {
                    display.set(x, y, 1);
                }
            }
        }

        // Rotate row
        else if parts[1] == "row" {
            let row: usize = parts[2].split("=")
                .collect::<Vec<&str>>()
                .last().unwrap()
                .parse().unwrap();
            let amount: usize = parts.last().unwrap().parse().unwrap();
            display.rotate_row(row, amount);
        }

        // Rotate column
        else {
            let column: usize = parts[2].split("=")
                .collect::<Vec<&str>>()
                .last().unwrap()
                .parse().unwrap();
            let amount: usize = parts.last().unwrap().parse().unwrap();
            display.rotate_column(column, amount)
        }
    }
    display.show();
    display.active_pixels()
}


struct Display {
    x_len: usize,
    y_len: usize,
    vector: Vec<u8>
}

impl Display {
    fn new(x_len: usize, y_len: usize) -> Self {

        Display {
            x_len,
            y_len,
            vector: vec![0;x_len*y_len]
        }
    }

    fn get(&self, x: usize, y: usize) -> u8 {
        if x > self.x_len || y > self.y_len {
            panic!("Attempting to get value outside of Display x: {x}, y: {y}")
        }
        self.vector[x + y*self.x_len]
    }

    fn set(&mut self, x: usize, y: usize, value: u8) {
        if x > self.x_len || y > self.y_len {
            panic!("Attempting to set value outside of Display x: {x}, y: {y}")
        }
        self.vector[x + y*self.x_len] = value;
    }

    fn active_pixels(self) -> u8 {
        self.vector.iter().sum::<u8>()
    }

    fn rotate_row(&mut self, row: usize, amount: usize) {
        self.vector[row*self.x_len..(row+1)*self.x_len].rotate_right(amount);
    }

    fn rotate_column(&mut self, column: usize, amount: usize){
        let mut temp_vector = vec![];
        for row in 0..self.y_len {
            temp_vector.push(self.get(column, row))
        }
        temp_vector.rotate_right(amount);
        for (row, value) in enumerate(temp_vector) {
            self.set(column, row, value)
        }
    }
    fn show(&self) {
        let temp_vector: Vec<char> = self.vector.iter().map(|a|
            {match a {
                0 => ' ',
                1 => '#',
                _ => '*'
            }}).collect();
        for row in 0..self.y_len {
            println!("{:?}", &temp_vector[row*self.x_len..(row+1)*self.x_len].iter().collect::<String>())
        }
        println!();
    }
}