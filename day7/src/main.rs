extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn task1(s: &str) {
    let lines = s.lines();

    let mut dependencies: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut result = String::new();

    for line in lines {
        let re =
            Regex::new(r"^Step ([A-Z]) must be finished before step ([A-Z]) can begin.$").unwrap();
        let captures = re.captures(line).expect("Failed to capture the regexp");

        let source = captures.get(1).expect("Expect source option").as_str();
        let target = captures.get(2).expect("Expect target option").as_str();

        dependencies
            .entry(source)
            .and_modify(|e| e.push(target))
            .or_insert_with(|| vec![target]);

        dependencies.entry(target).or_insert_with(|| vec![]);

        println!("{} -> {}", source, target);
    }

    while dependencies.len() > 0 {
        let key = {
            let mut available_keys = dependencies
                .iter()
                .filter(|(source, _t)| !dependencies.values().any(|v| v.contains(source)))
                .map(|e| *e.0)
                .collect::<Vec<_>>();

            available_keys.sort();
            available_keys.get(0).unwrap().clone()
        };

        result.push_str(key);
        dependencies.remove(key);
    }

    println!("Result: {:#?}", result);
}

fn main() {
    let mut f = File::open("input.txt").expect("File not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    task1(&contents);
}
