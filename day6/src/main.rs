extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

fn measure_distance(p1: &Point, p2: &Point) -> i32 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

fn task1(s: &str) {
    let lines = s.lines();
    let mut points = Vec::new();
    let mut field = HashMap::new();
    let mut per_owner = HashMap::new();

    for line in lines {
        let re = Regex::new(r"^(\d+),\s(\d+)$").unwrap();
        let captures = re.captures(line).expect("Failed to capture the regexp");

        let point = Point {
            x: captures.get(1).unwrap().as_str().parse().unwrap(),
            y: captures.get(2).unwrap().as_str().parse().unwrap(),
        };

        points.push(point);
    }

    let x_start = points.iter().map(|p| p.x).min().unwrap();
    let x_end = points.iter().map(|p| p.x).max().unwrap();
    let y_start = points.iter().map(|p| p.y).min().unwrap();
    let y_end = points.iter().map(|p| p.y).max().unwrap();

    for x in x_start..=x_end {
        for y in y_start..=y_end {
            let coord = Point { x, y };
            let closest_distance = points
                .iter()
                .map(|p| measure_distance(p, &coord))
                .min()
                .unwrap();

            let owners = points
                .iter()
                .filter(|p| measure_distance(p, &coord) == closest_distance)
                .collect::<Vec<_>>();

            if owners.len() == 1 {
                per_owner
                    .entry(owners[0])
                    .and_modify(|e: &mut Vec<_>| e.push(coord.clone()))
                    .or_insert(vec![coord.clone()]);
            }

            if owners.len() == 0 {
                println!("WTF: {:?}", coord);
            }

            field.insert(coord, owners.clone());
        }
    }

    let mut areas = per_owner
        .values()
        .filter(|v| {
            !v.iter()
                .any(|p| p.x == x_start || p.x == x_end || p.y == y_start || p.y == y_end)
        }).map(|v| v.len())
        .collect::<Vec<_>>();

    areas.sort();

    let biggest_area = areas.iter().max().unwrap();

    println!("Isolated area size: {}", biggest_area);
}

fn task2(s: &str) {
    let lines = s.lines();
    let mut points = Vec::new();
    let mut field = Vec::new();

    for line in lines {
        let re = Regex::new(r"^(\d+),\s(\d+)$").unwrap();
        let captures = re.captures(line).expect("Failed to capture the regexp");

        let point = Point {
            x: captures.get(1).unwrap().as_str().parse().unwrap(),
            y: captures.get(2).unwrap().as_str().parse().unwrap(),
        };

        points.push(point);
    }

    let x_start = points.iter().map(|p| p.x).min().unwrap() - 200;
    let x_end = points.iter().map(|p| p.x).max().unwrap() + 200;
    let y_start = points.iter().map(|p| p.y).min().unwrap() - 200;
    let y_end = points.iter().map(|p| p.y).max().unwrap() + 200;

    for x in x_start..=x_end {
        for y in y_start..=y_end {
            let coord = Point { x, y };
            let total_distance: i32 = points.iter().map(|p| measure_distance(p, &coord)).sum();

            if total_distance < 10000 {
                field.push(coord);
            }
        }
    }

    println!("Safe area size: {:?}", field.len());
}

fn main() {
    let mut f = File::open("input.txt").expect("File not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    task1(&contents);
    task2(&contents);
}
