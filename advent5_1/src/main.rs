use std::char;
use std::collections::HashMap;
use std::io::prelude::*;
use utils;

fn main() {
    let mut file = utils::open_argv1();
    let mut original = String::new();
    file.read_to_string(&mut original)
        .expect("couldn't read file");
    let original = original.trim().to_string();
    let reacted = react(&original);
    println!("Remaining units: {}", reacted.len());

    // Part 2
    // Brute-force solution: try removing every char a-z, seeing which is shortest.
    let letters = String::from("abcdefghijklmnopqrstuvwxy");
    let mut replaced_lengths = HashMap::new();

    for c in letters.chars() {
        let replaced = original.replace(c, "").replace(c.to_ascii_uppercase(), "");
        let reacted = react(&replaced);
        replaced_lengths.insert(c, reacted.len());
    }
    let (removed_char, resulting_length) = replaced_lengths.iter().min_by_key(|k| k.1).unwrap();
    println!(
        "Removing '{}' yields shortest, length {}",
        removed_char, resulting_length
    );
}

fn react(s: &String) -> String {
    let mut v: Vec<char> = s.chars().collect();

    let mut index = 0;
    while index < v.len() - 1 {
        let this_char = v[index];
        let next_char = v[index + 1];
        if reacts(this_char, next_char) {
            // remove this and the next char
            // not a typo: once we remove @ index, the next char slides down to become index
            v.remove(index);
            v.remove(index);

            // take a step back to hit new reactions - but not if it'd cause an underflow.
            if index > 0 {
                index -= 1;
            }
        } else {
            index += 1;
        }
    }

    return v.into_iter().collect::<String>();
}

// Does char a react with char b?
// Returns true if a and b are of opposite case.
fn reacts(a: char, b: char) -> bool {
    let char_matches = a.to_ascii_lowercase() == b.to_ascii_lowercase();
    let case_mismatch =
        (a.is_uppercase() && b.is_lowercase()) || (a.is_lowercase() && b.is_uppercase());
    return char_matches && case_mismatch;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_react() {
        let original = String::from("dabAcCaCBAcCcaDA");
        let expected = String::from("dabCBAcaDA");
        assert_eq!(expected, react(&original));
    }

    #[test]
    fn test_react_match_at_end() {
        let original = String::from("dabAcCaCBAcCcaDAa");
        let expected = String::from("dabCBAcaD");
        assert_eq!(expected, react(&original));
    }

    #[test]
    fn test_react_match_at_start() {
        let original = String::from("DdabAcCaCBAcCcaDAa");
        let expected = String::from("abCBAcaD");
        assert_eq!(expected, react(&original));
    }

    #[test]
    fn test_reacts() {
        assert_eq!(reacts('a', 'A'), true);
        assert_eq!(reacts('a', 'a'), false);
        assert_eq!(reacts('A', 'a'), true);
        assert_eq!(reacts('A', 'B'), false);
    }
}
