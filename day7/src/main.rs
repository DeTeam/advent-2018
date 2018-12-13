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

fn get_timing(key: &str) -> i32 {
    let base = "A".as_bytes()[0];
    let current = key.as_bytes()[0];

    (current - base + 1) as i32 + 60
}

fn task2(s: &str) {
    let lines = s.lines();


    const MAX_WORKERS: usize = 5;
    let mut dependencies: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut workers: Vec<(&str, i32)> = Vec::new();
    let mut result = String::new();
    let mut required_time: i32 = 0;

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
    }

    while dependencies.len() > 0 || workers.len() > 0  {
        println!("Workers before: {:?}", workers);
        {
            if let Some(next_timing) = workers.iter().map(|w| w.1).min() {
                required_time += next_timing;


                for worker in workers.clone().iter() {
                    workers.retain(|w| w != worker);

                    if worker.1 <= next_timing {
                        dependencies.remove(worker.0);
                        result.push_str(worker.0);
                    } else {
                        workers.push((worker.0, worker.1 - next_timing));
                    }
                }
            }
        }
        println!("Workers after: {:?}", workers);

        let next_keys = {
            let mut next_keys = dependencies
                .iter()
                .filter(|(source, _t)|
                    !dependencies.values().any(|v| v.contains(source)) && !workers.iter().any(|w| &&w.0 == source)
                )
                .map(|e| *e.0)
                .take(MAX_WORKERS - workers.len())
                .collect::<Vec<_>>();
            next_keys.sort();

            next_keys
        };
        
        for key in next_keys {
            workers.push((key, get_timing(key)));
        }
    }
    println!("Result: {:?}", result);
    println!("Required time: {:?}", required_time);
}

fn main() {
    let mut f = File::open("input.txt").expect("File not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    task1(&contents);
    task2(&contents);
}
