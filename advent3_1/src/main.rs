use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    let file = File::open(&path).expect(&format!("can't open {:?}", path));
    let buffer = BufReader::new(file);

    // represet the full fabric as a hashset of (x,y) coords
    // an existing value of (x,y) means there exists a claim at this location
    let mut fabric = HashSet::new();

    // represet overlaps as a similar hashset, but just the coords with an overlap
    // this avoids counting overlaps twice.
    let mut overlaps = HashSet::new();

    for line in buffer.lines() {
        let claim = Claim::from_string(&line.unwrap());

        for x in claim.top .. claim.top + claim.height {
            for y in claim.left .. claim.left + claim.width {
                if fabric.contains(&(x,y)) {
                    overlaps.insert((x,y));
                } else {
                    fabric.insert((x,y));
                }
            }
        }
    }

    println!("Overlapping Count: {}", overlaps.len());
}


#[derive(Debug)]
struct Claim {
    number: u32,
    left: u32,
    top: u32,
    width: u32,
    height: u32,
}

impl Claim {
    fn from_string(s: &str) -> Claim {
        // parse a string like "#1338 @ 925,820: 19x19" into a Claim

        let re = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
        let caps = re.captures(s).expect("invalid claim str");

        // this is ugly af, what am I doing wrong?
        return Claim {
            number: caps.get(1).unwrap().as_str().parse().unwrap(),
            left: caps.get(2).unwrap().as_str().parse().unwrap(),
            top: caps.get(3).unwrap().as_str().parse().unwrap(),
            width: caps.get(4).unwrap().as_str().parse().unwrap(),
            height: caps.get(5).unwrap().as_str().parse().unwrap(),
        };
    }
}
