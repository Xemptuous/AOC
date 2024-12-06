#!/usr/bin/env rust-script
const W: usize = 140;
const H: usize = 140;

fn xy_idx(x: usize, y: usize) -> usize {
    x * W + y
}

fn main() {
    let input: Vec<u8> = std::fs::read("inputs/d4.in")
        .expect("Couldn't open file.")
        .into_iter()
        .filter(|&b| !b.is_ascii_whitespace())
        .collect();

    let mut result = 0;
    for x in 1..W - 1 {
        for y in 1..H - 1 {
            if input[xy_idx(x, y)] != b'A' {
                continue;
            }

            let diagonal_left: Vec<u8> = vec![
                input[xy_idx(x - 1, y + 1)],
                input[xy_idx(x, y)],
                input[xy_idx(x + 1, y - 1)],
            ];
            let diagonal_right: Vec<u8> = vec![
                input[xy_idx(x - 1, y - 1)],
                input[xy_idx(x, y)],
                input[xy_idx(x + 1, y + 1)],
            ];

            let left_word = String::from_utf8_lossy(&diagonal_right);
            let right_word = String::from_utf8_lossy(&diagonal_left);

            if (right_word == "SAM" || right_word == "MAS")
                && (left_word == "SAM" || left_word == "MAS")
            {
                result += 1;
            }
        }
    }
    println!("{}", result);
}
