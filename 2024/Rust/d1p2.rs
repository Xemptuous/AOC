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
    let (left, right) = get_numbers_from_file()?;

    // create occurences in right list
    let mut r_map: HashMap<&i32, i32> = HashMap::new();
    for num in right.iter() {
        if !r_map.contains_key(num) {
            r_map.insert(num, 1);
            continue;
        }
        let e = r_map.get(num).unwrap();
        r_map.insert(num, e + 1);
    }

    // get sums of similarity score
    let mut sum = 0;
    for num in left.iter() {
        if let Some(n) = r_map.get(num) {
            sum += n * num;
        }
    }
    println!("{}", sum);
    Ok(())
}
