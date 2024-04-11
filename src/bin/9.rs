use std::str::Chars;
use std::time::Instant;
use itertools::Itertools;
use adventofcode2016::aoc_reader;

fn main() {
    let content = aoc_reader(9);
    let start = Instant::now(); // skip file IO in timing
    let result1 = decompress(content.clone());
    let result2 = decompress_recursive(content);

    let duration = start.elapsed();
    println!(
        "Part 1 result: {result1}\n\
         Part 2 result: {result2}"
    );
    println!("Execution took {:?}", duration);
}


fn decompress(data: String) -> usize {
    let mut sum = 0;
    let mut char_iterator = data.chars().into_iter();
    while let Some(char) = char_iterator.next() {
        if char == '(' {
            sum += collect_sequence(&mut char_iterator);
        }
        else {
            sum += 1;
        }
    }
    sum
}

fn collect_sequence(chars: &mut Chars) -> usize {
    let temp_string = chars.take_while_ref(|a| a != &')').collect::<String>();

    let (length, repeat) = temp_string.split('x')
        .map(|a| a.parse::<usize>().unwrap())
        .collect_tuple().unwrap();

    chars.dropping(length);  // skip characters that are decompressed
    length * repeat
}

fn decompress_recursive(string: String) -> usize {
    let mut sum = 0;
    let mut char_iterator = string.chars().into_iter();
    while let Some(char) = char_iterator.next() {
        if char == '(' {
            let (length, repeat) = char_iterator
                .take_while_ref(|a| a != &')')
                .collect::<String>()
                .split('x')
                .map(|a| a.parse::<usize>().unwrap())
                .collect_tuple().unwrap();

            let temp_string = char_iterator.by_ref().skip(1) // skip ')'
                .take(length).collect::<String>();
            sum += repeat * decompress_recursive(temp_string);
        }
        else {
            sum += 1;
        }
    }
    sum
}

