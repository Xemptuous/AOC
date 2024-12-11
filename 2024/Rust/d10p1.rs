use std::collections::VecDeque;

const DELTAS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];

fn main() {
    let grid = std::fs::read("inputs/d10.in").expect("Couldn't open file");

    let (h, w) = (
        grid.iter().filter(|b| b.is_ascii_whitespace()).count(),
        grid.iter().position(|b| b.is_ascii_whitespace()).unwrap() + 1,
    );
    let wi32 = w as i32;

    // // build adjacency matrix of possible paths
    let mut adj_list: Vec<Vec<bool>> = vec![vec![false; w * h]; w * h];
    let mut zeros: Vec<usize> = Vec::with_capacity(w * h);
    let mut nines: Vec<usize> = Vec::with_capacity(w * h);
    for x in 0..w {
        for y in 0..h {
            let idx = y * w + x;
            // Can't start at a '.'
            // Keep track of 0s and 9s for start and end checks
            match grid[idx] {
                b'.' => continue,
                b'0' => zeros.push(idx),
                b'9' => nines.push(idx),
                _ => {}
            };
            for (dx, dy) in DELTAS {
                let new_idx = ((y as i32 + dy) * wi32 + (x as i32 + dx)) as usize;

                if let Some(&new_c) = grid.get(new_idx) {
                    // can't move into a '.'
                    if new_c == b'.' {
                        continue;
                    }
                    if grid[idx].abs_diff(new_c) == 1 && new_c > grid[idx] {
                        adj_list[idx][new_idx] = true;
                    }
                }
            }
        }
    }

    // go through BFS and check all 0s to every possible 9
    let mut result = 0;
    for i in zeros {
        let mut visited = vec![false; w * h];
        let mut queue: VecDeque<usize> = VecDeque::new();

        // set start as visited
        visited[i] = true;
        queue.push_back(i);

        while let Some(curr) = queue.pop_front() {
            for (i, &adj) in adj_list[curr].iter().enumerate() {
                if adj && !visited[i] {
                    visited[i] = true;
                    queue.push_back(i);
                }
            }
        }
        // score for each successful path
        let mut score = 0;
        for j in nines.iter() {
            if visited[*j] {
                score += 1;
            }
        }
        result += score;
    }
    println!("{}", result);
}
