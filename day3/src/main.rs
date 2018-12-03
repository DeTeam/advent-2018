extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct Claim {
    id: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

fn task(s: &str) {
    let lines = s.lines();
    let mut field: HashMap<(i32, i32), Vec<i32>> = HashMap::new();
    let mut claims = HashMap::new();

    for line in lines {
        let re = Regex::new(r"^#(\d+)\s@\s(\d+),(\d+): (\d+)x(\d+)$").unwrap();
        let captures = re.captures(line).expect("Failed to capture the regexp");
        // TODO: extract in a fn?
        let claim = Claim {
            id: captures.get(1).unwrap().as_str().parse().unwrap(),
            x: captures.get(2).unwrap().as_str().parse().unwrap(),
            y: captures.get(3).unwrap().as_str().parse().unwrap(),
            width: captures.get(4).unwrap().as_str().parse().unwrap(),
            height: captures.get(5).unwrap().as_str().parse().unwrap(),
        };

        let x_start = claim.x;
        let x_end = x_start + claim.width;
        let y_start = claim.y;
        let y_end = y_start + claim.height;

        for i in x_start..x_end {
            for j in y_start..y_end {
                let coords = (i, j);

                field
                    .entry(coords)
                    .and_modify(|v| v.push(claim.id))
                    .or_insert(vec![claim.id]);
            }
        }
    }

    let count = field
        .iter()
        .filter(|x| x.1.len() > 1)
        .map(|x| x.0)
        .collect::<Vec<_>>()
        .len();

    println!("Amount of overlapping claims: {}", count);

    for ids in field.values() {
        if ids.len() > 1 {
            for id in ids {
                claims.insert(id, false);
            }
        } else {
            for id in ids {
                claims.entry(id).or_insert(true);
            }
        }
    }

    let proper_claims = claims
        .iter()
        .filter(|x| *x.1)
        .map(|x| x.0)
        .collect::<Vec<_>>();

    println!("Proper claims: {:#?}", proper_claims);
}

fn main() {
    let mut f = File::open("input.txt").expect("File not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    task(&contents);
}
