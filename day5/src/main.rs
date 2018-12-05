use std::fs::File;
use std::io::prelude::*;

fn should_destroy(a: char, b: char) -> bool {
    (a != b) && (a.to_ascii_uppercase() == b.to_ascii_uppercase())
}

fn collapse(s: &String) -> usize {
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

    to.len()
}

fn task(s: &String) {
    let input = s.trim().to_string();
    let default_length = collapse(&input);
    println!("Length by default: {}", default_length);

    let chars = [
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r",
        "s", "t", "u", "v", "w", "x", "y", "z",
    ];

    let mut variants = chars
        .iter()
        .map(|c| {
            let variant = input
                .clone()
                .replace(c, "")
                .replace(&c.to_ascii_uppercase(), "");

            collapse(&variant)
        }).collect::<Vec<_>>();

    variants.sort();

    let winner = variants.get(0).unwrap();

    println!("Shortest polymer: {}", winner);
}

fn main() {
    let mut f = File::open("input.txt").expect("File not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    task(&contents);
}
