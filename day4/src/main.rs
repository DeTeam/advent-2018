use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
enum Action {
    ShiftStarted(i32),
    FellAsleep,
    WokeUp,
}

#[derive(Debug)]
struct ParsedLine {
    date: String,
    minute: i32,
    action: Action,
}

fn process_line(line: &str) -> ParsedLine {
    ParsedLine {
        date: String::from("1518-10-26"),
        minute: 10,
        action: Action::ShiftStarted(200),
    }
}

fn task(s: &str) {
    println!("{}", s);

    let mut lines = s.lines().collect::<Vec<_>>().clone();
    lines.sort();

    for line in lines.iter() {
        println!("Line: {}", line);
        let pl = process_line(line);
        println!("Processed: {:#?}", pl);
    }
}

fn main() {
    let mut f = File::open("input.txt").expect("File not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    task(&contents);
}
