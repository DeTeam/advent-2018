#[macro_use]
extern crate nom;
extern crate chrono;

use chrono::prelude::*;
use nom::digit;
use std::collections::HashMap;
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

#[derive(Debug)]
struct Guard {
    id: i32,
    minutes: HashMap<i32, i32>,
    total_asleep: i32,
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
    let mut guards: HashMap<i32, Guard> = HashMap::new();
    let mut guard_id = None;
    let mut start = None;

    let mut lines = s.lines().collect::<Vec<_>>();

    lines.sort();

    let logs = lines
        .iter()
        .filter_map(|line| process_line(line).ok())
        .map(|x| x.1);

    for log in logs {
        match log.action {
            Action::ShiftStarted(id) => {
                guards.entry(id).or_insert(Guard {
                    id,
                    minutes: HashMap::new(),
                    total_asleep: 0,
                });

                guard_id = Some(id);
            }
            Action::FellAsleep => {
                start = Some(log.minute);
            }
            Action::WokeUp => {
                let pair = start.and_then(|s| guard_id.and_then(|i| Some((i, s))));
                match pair {
                    Some((id, start_m)) => {
                        let end = log.minute;
                        start = None;

                        let range = start_m..end;
                        for m in range {
                            guards.entry(id).and_modify(|e| {
                                e.minutes.entry(m).and_modify(|m| *m += 1).or_insert(1);
                                e.total_asleep += 1;
                            });
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    let mut scores = guards.values().clone().into_iter().collect::<Vec<_>>();
    scores.sort_by(|a, b| b.total_asleep.cmp(&a.total_asleep));
    let sleepy_guard = scores.get(0).unwrap();

    let mut minutes = sleepy_guard.minutes.iter().collect::<Vec<_>>();
    minutes.sort_by(|a, b| b.1.cmp(a.1));
    let lucky_minute = minutes.get(0).unwrap().0;

    println!(
        "Sleepy guard: {:?}, lucky minute: {:?}, result: {:?}",
        sleepy_guard.id,
        lucky_minute,
        sleepy_guard.id * lucky_minute
    );
}

fn main() {
    let mut f = File::open("input.txt").expect("File not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    task(&contents);
}
