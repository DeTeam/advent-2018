use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

fn task1(s: &str) {
    let lines = s.lines();
    let mut result: i32 = 0;

    for l in lines {
        let sign = l.get(0..1).expect("Invalid input");
        let value: i32 = l.get(1..).expect("Invalid input").parse().unwrap();

        // println!("Sign: {}, Value: {}", sign, value);

        if sign == "+" {
            result = result + value;
        } else {
            result = result - value;
        }
    }
    println!("First task's result: {}", result);
}

fn task2(s: &str) {
    let lines = s.lines();

    let mut results: HashSet<i32> = HashSet::new();
    let mut result: i32 = 0;

    for l in lines.cycle() {
        let sign = l.get(0..1).expect("Invalid input");
        let value: i32 = l.get(1..).expect("Invalid input").parse().unwrap();

        // println!("Sign: {}, Value: {}", sign, value);

        if sign == "+" {
            result = result + value;
        } else {
            result = result - value;
        }

        if results.contains(&result) {
            println!("Second task's result: {}", result);
            break;
        } else {
            results.insert(result);
        }
    }
}

fn main() {
    // `mut` here, because `read_to_string` takes `&mut self`
    let mut f = File::open("input.txt").expect("File not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    task1(&contents);
    task2(&contents);
}
