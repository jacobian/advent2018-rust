use std::io::prelude::*;
use std::io::BufReader;
use utils;

fn main() {
    let file = utils::open_argv1();

    // use a buffer to read lines
    // https://doc.rust-lang.org/std/fs/struct.File.html
    // https://doc.rust-lang.org/std/io/trait.BufRead.html#method.lines
    let buffer = BufReader::new(file);
    let mut frequency = 0;
    for line in buffer.lines() {
        let val: i32 = match line.unwrap().parse() {
            Err(_) => continue,
            Ok(n) => n,
        };
        frequency += val;
    }
    println!("Part 1 result (final frequency): {}", frequency);
}
