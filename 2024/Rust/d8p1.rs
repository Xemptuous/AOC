#!/usr/bin/env rust-script
use std::collections::HashMap;

fn main() {
    let grid: Vec<u8> = std::fs::read("inputs/d8.in").expect("Couldn't open file");
    let mut antinode_grid = grid.clone();

    let h = grid.iter().filter(|&b| b.is_ascii_whitespace()).count();
    let w = grid.iter().position(|&b| b.is_ascii_whitespace()).unwrap() + 1;

    // Go through the grid once to get all unique antennas and their positions
    // Store chars as keys and all their indexes in vectors
    let mut antenna_locations: HashMap<u8, Vec<Antenna>> = HashMap::new();
    for (i, ch) in grid.iter().enumerate() {
        if *ch == b'.' || ch.is_ascii_whitespace() {
            continue;
        }
        antenna_locations
            .entry(*ch)
            .or_default()
            .push(Antenna::new(i, w));
    }

    // go through each unique character's antennas
    for (_, antennas) in antenna_locations.iter() {
        // Go over all possible combinations
        // Could reduce runtime if skipping over already-done pairs
        for a1 in antennas {
            for a2 in antennas.iter().skip(1) {
                // if the nodes are the same
                if a1.index == a2.index {
                    continue;
                }

                // get deltas for movement
                let dx = a1.x - a2.x;
                let dy = a1.y - a2.y;

                // get both antinode positions and add to grid if applicable
                if let Some(i) = get_antinode_position(a1, dx, dy, w, h) {
                    antinode_grid[i] = b'#';
                }
                if let Some(i) = get_antinode_position(a2, -dx, -dy, w, h) {
                    antinode_grid[i] = b'#';
                }
            }
        }
    }
    let result = antinode_grid.iter().filter(|&&b| b == b'#').count();
    println!("{}", result);
}

struct Antenna {
    index: usize,
    x: i32,
    y: i32,
}

impl Antenna {
    pub fn new(index: usize, width: usize) -> Self {
        Antenna {
            index,
            x: (index % width) as i32,
            y: (index / width) as i32,
        }
    }
}

fn get_antinode_position(a: &Antenna, dx: i32, dy: i32, w: usize, h: usize) -> Option<usize> {
    // get antinode positions
    let ax = a.x + dx;
    let ay = a.y + dy;

    // if OOB
    if ax >= (w - 1) as i32 || ax < 0 {
        return None;
    }
    if ay >= h as i32 || ay < 0 {
        return None;
    }

    Some(xy_to_idx(ax as usize, ay as usize, w))
}

fn xy_to_idx(x: usize, y: usize, width: usize) -> usize {
    y * width + x
}

fn draw_grid(grid: &[u8], width: usize, height: usize) {
    for y in 0..width {
        let mut buf = String::new();
        for x in 0..height {
            let idx = xy_to_idx(x, y, width);
            if idx < grid.len() {
                let c = grid[idx] as char;
                if c.is_ascii_whitespace() {
                    continue;
                }
                buf.push(c);
            }
        }
        println!("{}", buf);
    }
}

fn draw_antinode_grid(antinode_grid: &[u8], width: usize, height: usize) {
    for y in 0..width - 1 {
        let mut buf = String::new();
        for x in 0..height {
            buf.push(antinode_grid[xy_to_idx(x, y, width)] as char);
        }
        println!("{}", buf);
    }
}
