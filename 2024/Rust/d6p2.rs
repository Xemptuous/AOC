#!/usr/bin/env rust-script
const W: usize = 131;

#[derive(PartialEq, Debug, Clone)]
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

fn try_move(grid: &[u8], x: usize, y: usize, direction: Direction) -> Option<usize> {
    use Direction::*;
    let move_to = match direction {
        Up => {
            if x == 0 {
                return None;
            }
            xy_to_idx(x - 1, y)
        }
        Down => {
            if x == W - 2 {
                return None;
            }
            xy_to_idx(x + 1, y)
        }
        Left => {
            if y == 0 {
                return None;
            }
            xy_to_idx(x, y - 1)
        }
        Right => {
            let to = xy_to_idx(x, y + 1);
            if grid[to].is_ascii_whitespace() {
                return None;
            }
            to
        }
    };
    Some(move_to)
}

fn main() {
    use Direction::*;
    let mut grid: Vec<u8> = std::fs::read("inputs/d6.in").expect("Couldn't open file");

    let mut direction = Direction::Up;
    let (start_position, _) = grid
        .iter()
        .enumerate()
        .find(|(_, &c)| c == b'^')
        .expect("Couldn't find starting position");

    // first run through to find path
    let mut position = start_position;
    loop {
        grid[position] = b'X'; // mark current position as visited
        let (x, y) = idx_to_xy(&position);
        match try_move(&grid, x, y, direction.clone()) {
            None => break,
            Some(move_to) => {
                if grid[move_to] == b'#' {
                    direction = match direction {
                        Up => Right,
                        Right => Down,
                        Down => Left,
                        Left => Up,
                    };
                } else {
                    position = move_to;
                }
            }
        }
    }

    // only place obstacle in visited areas
    let visited_grid: Vec<usize> = grid
        .iter()
        .enumerate()
        .filter_map(|(i, &c)| if c == b'X' { Some(i) } else { None })
        .collect();

    // second run through creating an obstacle at each tile to find loops
    let mut result = 0;
    for idx in visited_grid.iter() {
        if *idx == start_position {
            continue; // cannot place at guard's starting position
        }
        // reset direction and position
        direction = Direction::Up;
        position = start_position;

        // use array of blocks hit, as he will never hit it gain
        let mut walls_hit_grid: Vec<Vec<Direction>> = vec![Vec::new(); grid.len()];

        let og_char = grid[*idx];
        grid[*idx] = b'#'; // replace tile with an obstacle

        loop {
            let (x, y) = idx_to_xy(&position);
            match try_move(&grid, x, y, direction.clone()) {
                None => break,
                Some(move_to) => {
                    // check if wall has been hit in current direction
                    if grid[move_to] == b'#' {
                        // if wall previously hit in this direction, is loop
                        if walls_hit_grid[move_to].contains(&direction.clone()) {
                            result += 1;
                            break;
                        }
                        // log wall + direction, and rotate
                        walls_hit_grid[move_to].push(direction.clone());
                        direction = match direction {
                            Up => Right,
                            Right => Down,
                            Down => Left,
                            Left => Up,
                        };
                    } else {
                        position = move_to;
                    }
                }
            }
        }
        grid[*idx] = og_char; // reset obstacle inserted
    }
    println!("{}", result);
}
