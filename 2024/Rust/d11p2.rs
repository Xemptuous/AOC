use std::collections::{HashMap, LinkedList};

fn main() {
    let string = std::fs::read_to_string("inputs/d11.in").expect("Couldn't read file");
    let numbers: Vec<usize> = string
        .split_whitespace()
        .map(|v| v.parse())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let mut stones: HashMap<usize, usize> = numbers
        .into_iter()
        .map(|v| (v, 1))
        .collect::<HashMap<usize, usize>>();

    for _ in 0..75 {
        let mut next = HashMap::new();
        for (stone, count) in stones {
            let string = stone.to_string();
            // Rule 1: if 0, then 1
            let blinked = if stone == 0 {
                vec![1]
            }
            // Rule 2: if even number, split into 2. No leading zeros (for right)
            else if string.len() % 2 == 0 {
                let (left, right) = string.split_at(string.len() / 2);
                vec![
                    left.parse::<usize>().unwrap(),
                    right.parse::<usize>().unwrap(),
                ]
            }
            // Rule 3: else, * 2024
            else {
                vec![stone * 2024]
            };
            for blink in blinked {
                let next_count = next.entry(blink).or_default();
                *next_count += count;
            }
        }
        stones = next;
    }

    let result: usize = stones.values().sum();
    println!("{}", result);
}
