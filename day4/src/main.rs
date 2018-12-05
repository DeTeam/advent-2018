#[macro_use]
extern crate nom;
extern crate chrono;

use chrono::prelude::*;
use nom::digit;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

#[derive(Debug)]
enum Action {
    ShiftStarted(i32),
    FellAsleep,
    WokeUp,
}

#[derive(Debug)]
struct ParsedLine {
    date: NaiveDate,
    minute: i32,
    action: Action,
}

named!(process_action<&str,Action>,
    alt_complete!(
        tag!("wakes") => { |_| Action::WokeUp } |
        tag!("falls") => { |_| Action::FellAsleep } |
        do_parse!(
            tag!("Guard #") >>
            guard: map_res!(digit, |x| FromStr::from_str(x)) >>
            (
                Action::ShiftStarted(guard)
            )
        )
    )
);

fn normalize_date(date: &str, hour: &str) -> NaiveDate {
    let current_date: NaiveDate = FromStr::from_str(date).unwrap();

    match hour {
        "23" => current_date.succ(),
        _ => current_date,
    }
}

fn normalize_minute(hour: &str, minute: &str) -> i32 {
    match hour {
        "23" => 0,
        _ => FromStr::from_str(minute).unwrap(),
    }
}

named!(process_line<&str,ParsedLine>,
    ws!(
        do_parse!(
                    tag!("[") >>
            date:   take!(10) >>
            hour:   take!(2) >>
                    tag!(":") >>
            minute: take!(2) >>
                    tag!("]") >>
            action: process_action >>
            (
                ParsedLine {
                    date: normalize_date(date, hour),
                    minute: normalize_minute(hour, minute),
                    action: action,
                }
            )
        )
    )
);

fn task(s: &str) {
    let lines = s.lines();

    for line in lines.take(10) {
        let parsed_line = process_line(line);
        println!("{:#?}", parsed_line);
    }
}

fn main() {
    let mut f = File::open("input.txt").expect("File not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    task(&contents);
}
