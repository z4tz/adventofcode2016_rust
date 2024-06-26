use std::collections::HashMap;
use std::time::Instant;
use adventofcode2016::aoc_reader;

static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e',
    'f', 'g', 'h', 'i', 'j',
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't',
    'u', 'v', 'w', 'x', 'y',
    'z',
];

fn main() {
    let content = aoc_reader(4);
    let start = Instant::now(); // skip file IO in timing
    let (result1, result2) = sector_sum(&content);
    let duration = start.elapsed();
    println!(
        "Part 1 result: {result1}\n\
         Part 2 result: {result2}"
    );
    println!("Execution took {:?}", duration)
}

fn sector_sum(data: &str) -> (u32, u32) {
    let mut real_ids = 0;
    let mut north_pole_id: u32 =0;
    for line in data.lines() {
        let sector = Sector::from_string(line);

        if sector.checksum == most_common_chars(&sector.name) {
            real_ids += sector.id;
            if north_pole_id == 0 && sector.decrypted().contains("northpole") {
                north_pole_id = sector.id.clone();
            }
        }
    }
    (real_ids, north_pole_id)
}


fn most_common_chars(string: &String) -> String {
    let mut letter_counts: HashMap<char,i32> = HashMap::new();
    for c in string.chars() {
        *letter_counts.entry(c).or_insert(0) += 1;
    }
    let mut count_vec: Vec<(&char,&i32)> = letter_counts.iter().collect();
    // sort by occurrence then alphabetical
    count_vec.sort_by(|a, b | {
        a.1.cmp(b.1).reverse()
            .then(a.0.cmp(b.0))
    });

    count_vec.truncate(5);
    count_vec.iter().map(|a| a.0).collect()
}

#[derive(Debug)]
struct Sector{
    name: String,
    id: u32,
    checksum: String
}
impl Sector {
    fn from_string(string: &str) -> Sector {
        let mut parts = string.split("[");
        let (temp_name, temp_id) = parts.next().unwrap().rsplit_once("-").unwrap();
        let checksum = parts.next().unwrap().strip_suffix("]").unwrap().to_string();

        let name = temp_name.replace("-","");
        let id = temp_id.parse::<u32>().unwrap();

        Sector{
            name,
            id,
            checksum
        }
    }

    fn decrypted(&self) -> String {
        let mut decrypted_chars: Vec<char> = vec![];
        for char in self.name.chars() {
            let index = (ASCII_LOWER.iter().position( |x| x == &char).unwrap() + self.id as usize) % ASCII_LOWER.len();
            decrypted_chars.push(ASCII_LOWER[index]);
        }
        decrypted_chars.iter().collect()
    }
}

