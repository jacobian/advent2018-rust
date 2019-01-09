use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashSet;
use utils;

fn main () {
    // read list of frequency changes from file into a list
    // we're going to repeat over this list multiple times possibly
    // it feels like there should be an easier way to read lines into
    // a list of ints -- this feels like a lot of code.
    let file = utils::open_argv1();
    let buffer = BufReader::new(file);
    let mut changes:Vec<i32> = Vec::new();
    for line in buffer.lines() {
        let val:i32 = match line.unwrap().parse() {
            Err(_) => continue,
            Ok(num) => num
        };
        changes.push(val);
    }

    // until we see a repeated frequency, continue to loop over all
    // the changes, adding them to the accumulator (frequency) and
    // appending them to the seen-frequencies vector. Using a
    // loop and an index instead of an interator because we probably
    // need to loop multiple times.
    let mut index = 0;
    let mut frequency = 0;
    let mut seen_frequencies:HashSet<i32> = HashSet::new();
    loop {
        frequency += changes[index];
        if seen_frequencies.contains(&frequency) { break; }
        seen_frequencies.insert(frequency);
        index += 1;
        if index >= changes.len() {
            index = 0;
        }
    }
    println!("Part 2 result (first repeated frequency): {}", frequency)
}
