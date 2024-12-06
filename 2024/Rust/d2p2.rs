#!/usr/bin/env rust-script
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn is_safe(nums: Vec<i32>) -> bool {
    let asc = nums
        .windows(2)
        .all(|w| w[0] < w[1] && w[0].abs_diff(w[1]) <= 3);
    let desc = nums
        .windows(2)
        .all(|w| w[0] > w[1] && w[0].abs_diff(w[1]) <= 3);
    return asc || desc;
}

fn is_almost_safe(nums: Vec<i32>) -> bool {
    for i in 0..nums.len() {
        let mut n = nums.clone();
        n.remove(i);
        if is_safe(n) {
            return true;
        }
    }
    return false;
}

fn main() {
    let file = File::open("inputs/d2.in").expect("Couldn't open file");
    let reader = BufReader::new(file);

    let mut total_safe: i32 = 0;
    for line in reader.lines() {
        let line_str = line.expect("Couldn't read line");
        let report: Vec<i32> = line_str
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        if is_safe(report.clone()) || is_almost_safe(report) {
            total_safe += 1;
        }
    }
    println!("{}", total_safe);
}
