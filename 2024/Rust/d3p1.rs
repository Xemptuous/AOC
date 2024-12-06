#!/usr/bin/env rust-script
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn read_number(input: &str, curr: &mut usize, peek: &mut usize) -> i32 {
    let pos = *curr;
    let mut ch: char = input.as_bytes()[*curr].into();
    while ch.is_ascii_digit() {
        *curr += 1;
        *peek += 1;
        ch = input.as_bytes()[*curr].into();
    }
    let diff = *curr - pos;
    let s = &input[pos..pos + diff];
    return s.parse::<i32>().unwrap();
}

fn main() {
    let file = File::open("inputs/d3.in").expect("Couldn't open file");
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    reader
        .read_to_string(&mut line)
        .expect("Couldn't read line");
    let matches: Vec<_> = line.match_indices("mul(").collect();

    let mut sum = 0;
    for (start, _) in matches.iter() {
        let (mut curr, mut peek) = (start + 4, start + 5);

        let n1 = read_number(line.as_str(), &mut curr, &mut peek);

        if line.as_bytes()[curr] != b',' {
            continue;
        }
        curr += 1;
        peek += 1;

        let n2 = read_number(line.as_str(), &mut curr, &mut peek);

        if line.as_bytes()[curr] != b')' {
            continue;
        }
        sum += n1 * n2;
    }
    println!("{}", sum);
}
