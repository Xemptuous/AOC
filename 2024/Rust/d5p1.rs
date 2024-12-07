#!/usr/bin/env rust-script
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("inputs/d5.in").expect("Couldn't open file");
    let reader = BufReader::new(file);

    let mut iter = reader.lines();

    // build rules
    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    loop {
        let string = iter.next().unwrap().unwrap();
        if string.contains("|") {
            let mut items = string.split("|");
            let page = items.next().unwrap().parse::<i32>().unwrap();
            let rule = items.next().unwrap().parse::<i32>().unwrap();

            let e = rules.entry(page).or_default();
            if !e.contains(&rule) {
                e.push(rule);
            }
            continue;
        } else {
            break;
        }
    }

    let mut result = 0;
    'line: for update in iter {
        let string = update.expect("Couldn't read line");
        let pages: Vec<i32> = string
            .split(',')
            .map(|i| i.parse::<i32>().unwrap())
            .collect();

        // go through each page in the update line (reverse)
        for i in (0..pages.len()).rev() {
            if let Some(ruleset) = rules.get(&pages[i]) {
                // look backwards for anything that doesn't belong
                for j in (0..i).rev() {
                    if ruleset.contains(&pages[j]) {
                        continue 'line;
                    }
                }
            }
        }
        // update line is always odd
        result += pages.get((pages.len() - 1) / 2).unwrap();
    }
    println!("{}", result);
}
