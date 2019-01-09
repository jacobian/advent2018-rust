use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    let file = File::open(&path).expect(&format!("can't open {:?}", path));
    let buffer = BufReader::new(file);

    // represet the full fabric as a hashmap mapping (x,y) -> set of claims to that (x,y)
    // I'm not entirely clear on why I have to specify the types here but without it won't compile
    let mut fabric: HashMap<(u32, u32), HashSet<u32>> = HashMap::new();

    // claims that have no overlaps
    let mut pristine_claims = HashSet::new();

    for line in buffer.lines() {
        let claim = Claim::from_string(&line.unwrap());

        // at first when we encounter the claim, it's pristine because we've not yet looked for any overlaps
        pristine_claims.insert(claim.number);

        for x in claim.top..claim.top + claim.height {
            for y in claim.left..claim.left + claim.width {
                // if there's already claim(s) here, remove them from the pristine claims list
                // and remove this claim as well
                if fabric.contains_key(&(x, y)) {
                    for claim_id in fabric.get(&(x, y)).unwrap().iter() {
                        pristine_claims.remove(claim_id);
                    }
                    pristine_claims.remove(&claim.number);
                }

                // track this claim at the spot
                fabric
                    .entry((x, y))
                    .or_insert(HashSet::new())
                    .insert(claim.number);
            }
        }
    }

    println!("Pristine Claims: {:?}", pristine_claims);
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
