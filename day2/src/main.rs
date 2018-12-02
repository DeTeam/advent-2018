use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn get_score(s: &str) -> (bool, bool) {
    let mut letters = HashMap::new();
    for c in s.chars() {
        let counter = letters.entry(c).or_insert(0);
        *counter += 1;
    }

    let collected_values: Vec<_> = letters.values().collect();
    let mut values = collected_values.clone();

    values.sort();
    values.dedup();

    let final_values = values.clone();

    // OMG, refactor that part and understand the whole function better
    (final_values.contains(&&2), final_values.contains(&&3))
}

fn task1(s: &str) {
    let lines = s.lines();

    let mut threes = 0;
    let mut twos = 0;

    for line in lines {
        let result = get_score(line);

        twos = if result.0 { twos + 1 } else { twos };
        threes = if result.1 { threes + 1 } else { threes };
    }

    let result = threes * twos;
    println!("First task result: {}", result);
}

fn get_common_part(s1: &str, s2: &str) -> String {
    let mut result = String::new();
    for (c1, c2) in s1.chars().zip(s2.chars()) {
        if c1 == c2 {
            result.push(c1);
        }
    }

    result
}

fn task2(s: &str) {
    let lines = s.lines();
    'outer: for (i, l1) in lines.clone().enumerate() {
        let other_lines = lines.clone().skip(i);

        for l2 in other_lines {
            let common_part = &get_common_part(l1, l2);

            if l1.len() - common_part.len() == 1 {
                println!("Common part: {}", common_part);
                break 'outer;
            }
        }
    }
}

fn main() {
    let mut f = File::open("input.txt").expect("File not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    task1(&contents);
    task2(&contents);
}
