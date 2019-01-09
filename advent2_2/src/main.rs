use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use utils;

fn main() {
    let file = utils::open_argv1();
    let buffer = BufReader::new(file);

    // Need to_string here for reasons I don't totally undertand
    // otherwise get a "doesn't have a size known at compile-time" down below in the for loop.
    let box_ids: Vec<String> = buffer
        .lines()
        .map(|line| line.unwrap().to_string())
        .collect();

    'outer: for id1 in box_ids.iter() {
        for id2 in box_ids.iter() {
            let diff = diff_by_index(&id1, &id2);
            if diff.len() == 1 {
                println!("Nearly matching IDs: {} & {}", id1, id2);
                let mut common_chars = id1.clone();
                common_chars.remove(diff[0]);
                println!("Common chars: {}", common_chars);
                break 'outer;
            }
        }
    }
}

// diff two strings, returning the indexes that differ
// eg diff_by_index("abcde", "axcye") -> [1, 4]
fn diff_by_index(s1: &str, s2: &str) -> Vec<usize> {
    // dunno why I have to use usize here, if I don't the push() fails
    let mut diff: Vec<usize> = Vec::new();

    assert_eq!(s1.len(), s2.len());

    for (index, (c1, c2)) in s1.chars().zip(s2.chars()).enumerate() {
        if c1 != c2 {
            diff.push(index);
        }
    }

    return diff;
}
