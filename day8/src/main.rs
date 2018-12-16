#[macro_use]
extern crate nom;

use nom::{digit};
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

named!(parse_head<&str,(u32, u32)>,
    ws!(
        do_parse!(
            children:   map_res!(digit, |x| FromStr::from_str(x)) >>
            meta:       map_res!(digit, |x| FromStr::from_str(x)) >>
            (
                (children, meta)
            )
        )
    )
);

named!(simple_aggregate_meta<&str,u32>,
    ws!(
        do_parse!(
            head:  parse_head >>
            children_meta: dbg!(count!(
                simple_aggregate_meta,
                head.0 as usize
            )) >>
            own_metas: dbg!(count!(
                ws!(map_res!(digit, |x| u32::from_str(x))),
                head.1 as usize
            )) >>
            (own_metas.iter().chain(children_meta.iter()).fold(0, |acc, x| acc + x))
        )
    )
);

named!(indexed_aggregate_meta<&str,Vec<u32>>,
    ws!(
        do_parse!(
            head:  parse_head >>
            children_meta: dbg!(count!(
                indexed_aggregate_meta,
                head.0 as usize
            )) >>
            own_metas: dbg!(count!(
                ws!(map_res!(digit, |x| u32::from_str(x))),
                head.1 as usize
            )) >>
            (
                if head.0 > 0 {
                    own_metas.iter().filter_map(|&i| children_meta.get(i as usize - 1)).flatten().cloned().collect::<Vec<_>>()
                } else {
                    own_metas
                }
            )
        )
    )
);


fn task1(s: &str) {
    let result = simple_aggregate_meta(s).unwrap().1;
    println!("Result 1: {:#?}", result);

    let result = indexed_aggregate_meta(s).unwrap().1.iter().fold(0, |acc, x| acc + x);
    println!("Result 2: {:#?}", result);
}

fn main() {
    let mut f = File::open("input.txt").expect("File not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    task1(&contents);
}
