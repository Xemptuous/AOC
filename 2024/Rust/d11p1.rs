use std::collections::LinkedList;

fn main() {
    let string = std::fs::read_to_string("inputs/d11.in").expect("Couldn't read file");
    let numbers: Vec<String> = string.split_whitespace().map(String::from).collect();

    let mut head: LinkedList<String> = LinkedList::new();
    for number in numbers {
        head.push_back(number);
    }

    for _ in 0..25 {
        let iter = head.iter();
        let mut dummy: LinkedList<String> = LinkedList::new();
        for s in iter {
            // Rule 1: if 0, then 1
            if s == "0" {
                dummy.push_back(String::from("1"));
            }
            // Rule 2: if even number, split into 2. No leading zeros (for right)
            else if s.len() % 2 == 0 {
                let (left, right) = s.split_at(s.len() / 2);
                dummy.push_back(left.to_string());
                dummy.push_back(parse_right(right));
            }
            // Rule 3: else, * 2024
            else {
                let num = (s.parse::<u64>().unwrap() * 2024).to_string();
                dummy.push_back(num);
            }
        }
        head = dummy;
    }
    println!("{}", head.len());
}

fn parse_right(right: &str) -> String {
    if right.starts_with('0') {
        if !right.chars().all(|c| c == '0') {
            return String::from(right.trim_start_matches('0'));
        } else {
            return String::from("0");
        }
    }
    String::from(right)
}
