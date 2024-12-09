#!/usr/bin/env rust-script
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn permutations(n: usize) -> Vec<String> {
    if n == 0 {
        return vec![String::new()];
    }

    let smaller_permutations: Vec<String> = permutations(n - 1);
    let mut result: Vec<String> = Vec::new();

    // order permutations so that * comes before +
    // This well help us check later if running_total > total
    // so that we can break early
    for perm in &smaller_permutations {
        result.push(format!("*{}", perm));
    }
    for perm in &smaller_permutations {
        result.push(format!("+{}", perm));
    }

    result
}

fn main() {
    let file = File::open("inputs/d7.in").expect("Couldn't open file");
    let reader = BufReader::new(file);

    let mut result = 0;
    for line in reader.lines() {
        let line = line.expect("Couldn't read line");
        let colon_pos = line.find(':').unwrap();

        let total = &line[0..colon_pos].parse::<u64>().unwrap();
        let numbers: Vec<u64> = line[colon_pos + 1..]
            .trim_start()
            .split(' ')
            .map(|n| n.parse::<u64>().unwrap())
            .collect();

        // go through each possible permutation of operators
        'perm: for op_string in permutations(numbers.len() - 1) {
            let mut running_total: u64 = numbers[0]; // get first number
            let iter = numbers.iter().skip(1).zip(op_string.chars());

            for (num, op) in iter {
                running_total = match op {
                    '*' => running_total * num,
                    _ => running_total + num,
                };
                // due to ordered permutations, continue if too large
                // as other values will not be smaller due to left-right
                // evaluation (as opposed to precedence)
                if running_total > *total {
                    continue 'perm;
                }
                if running_total == *total {
                    result += total;
                    break 'perm;
                }
            }
        }
    }
    println!("{}", result);
}
