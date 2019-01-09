use std::collections::HashMap;
use std::collections::HashSet;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use utils;

fn main() {
    let file = utils::open_argv1();
    let buffer = BufReader::new(file);

    let mut twos_count = 0;
    let mut threes_count = 0;

    for line in buffer.lines() {
        let count = count_chars(&line.unwrap());
        let count_values: HashSet<_> = count.values().collect();
        if count_values.contains(&2) {
            twos_count += 1;
        }
        if count_values.contains(&3) {
            threes_count += 1;
        }
    }

    let checksum = twos_count * threes_count;
    println!("Checksum: {}", checksum);
}

// count the occurances of chars in a string
// returns a map mapping char -> count for each char in string
fn count_chars(s: &str) -> HashMap<char, u32> {
    let mut count = HashMap::new();
    for c in s.chars() {
        *count.entry(c).or_insert(0) += 1;
    }
    return count;
}

//
