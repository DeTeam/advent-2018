use std::fs::File;
use std::io::prelude::*;

fn should_destroy(a: char, b: char) -> bool {
    (a != b) && (a.to_ascii_uppercase() == b.to_ascii_uppercase())
}

fn task(s: &String) {
    let mut from = s.trim().clone().chars().rev().collect::<String>();
    let mut to = String::new();

    while from.len() > 0 {
        let right = from.pop().unwrap();

        if let Some(left) = to.pop() {
            if should_destroy(left, right) {
                if let Some(move_over) = to.pop() {
                    from.push(move_over);
                }
            } else {
                to.push(left);
                to.push(right);
            }
        } else {
            to.push(right);
        }
    }

    println!("Original length: {}", s.trim().len());
    println!("Leftover length: {}", to.len());
}

fn main() {
    let mut f = File::open("input.txt").expect("File not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    task(&contents);
}
