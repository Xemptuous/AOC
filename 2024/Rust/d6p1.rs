#!/usr/bin/env rust-script
const W: usize = 131;

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn idx_to_xy(i: &usize) -> (usize, usize) {
    let x = i / W;
    let y = i % W;
    (x, y)
}

fn xy_to_idx(x: usize, y: usize) -> usize {
    x * W + y
}

fn main() {
    let mut grid: Vec<u8> = std::fs::read("inputs/d6.in").expect("Couldn't open file");

    // starting direction
    let mut direction = Direction::Up;
    // starting position index
    let (mut position, _) = grid
        .iter()
        .enumerate()
        .find(|(_, &c)| c == b'^')
        .expect("Couldn't find starting position");

    // starting point visited
    grid[position] = b'X';

    use Direction::*;
    loop {
        let (x, y) = idx_to_xy(&position);
        let move_to = match direction {
            Up => xy_to_idx(x - 1, y),
            Down => xy_to_idx(x + 1, y),
            Left => xy_to_idx(x, y - 1),
            Right => xy_to_idx(x, y + 1),
        };
        let (new_x, _) = idx_to_xy(&move_to);

        // if moving into OOB
        if grid[move_to].is_ascii_whitespace() || new_x == W {
            break;
        }

        // rotate if moving into wall
        if grid[move_to] == b'#' {
            direction = match direction {
                Up => Right,
                Right => Down,
                Down => Left,
                Left => Up,
            };
            continue;
        }
        position = move_to; // move to new position
        grid[position] = b'X'; // mark current position as visited
    }
    let result = grid.iter().filter(|&&a| a == b'X').count();
    println!("{}", result);
}
