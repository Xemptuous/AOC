#!/usr/bin/env rust-script
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader, Result};

fn get_numbers_from_file() -> Result<(Vec<i32>, Vec<i32>)> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    for line in reader.lines() {
        let line_str = line?;
        let mut split = line_str.split_whitespace();
        let (l, r) = (split.next().unwrap(), split.next().unwrap());
        left.push(String::from(l).parse::<i32>().unwrap());
        right.push(String::from(r).parse::<i32>().unwrap());
    }
    Ok((left, right))
}

fn main() -> Result<()> {
    let (mut left, mut right) = get_numbers_from_file()?;

    // sort both lists
    left.sort();
    right.sort();

    // get difference of values
    let mut results: Vec<i32> = vec![0; left.len()];
    results.reserve_exact(left.len());
    for i in 0..left.len() {
        results[i] = if left[i] > right[i] {
            left[i] - right[i]
        } else {
            right[i] - left[i]
        };
    }
    let sum: i32 = results.iter().sum();
    println!("{}", sum);
    Ok(())
}
