use std::collections::{hash_map::Entry::Vacant, HashMap, VecDeque};

const DELTAS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];

fn main() {
    let grid = std::fs::read("inputs/d12.in").expect("Couldn't open file");

    let (h, w) = (
        grid.iter().filter(|b| b.is_ascii_whitespace()).count(),
        grid.iter().position(|b| b.is_ascii_whitespace()).unwrap(),
    );
    let hi32 = h as i32;
    let wi32 = w as i32;

    let plant_grid: Vec<u8> = grid
        .into_iter()
        .filter(|&b| !b.is_ascii_whitespace())
        .collect();

    let mut visited_vec: Vec<bool> = vec![false; plant_grid.len()];
    let mut plant_group_map: HashMap<u8, Vec<Vec<usize>>> = HashMap::new();

    for y in 0..w {
        for x in 0..h {
            let idx = y * w + x;

            if visited_vec[idx] {
                continue;
            }
            let ch = plant_grid[idx];

            // if we've visited this plant type before, create a new "group"
            if let Vacant(e) = plant_group_map.entry(ch) {
                e.insert(vec![vec![]]);
            } else {
                plant_group_map.get_mut(&ch).unwrap().push(Vec::new());
            }

            // queue to process all same plants
            let mut to_process: VecDeque<(&u8, i32, i32)> = VecDeque::new();
            to_process.push_back((&ch, x as i32, y as i32));

            while let Some((curr, nx, ny)) = to_process.pop_front() {
                let idx = ny as usize * w + nx as usize;
                visited_vec[idx] = true;
                let mut fence_count = 4;
                for (dx, dy) in DELTAS {
                    let (new_x, new_y) = (nx + dx, ny + dy);

                    // if OOB
                    if new_x < 0 || new_x >= wi32 || new_y < 0 || new_y >= hi32 {
                        continue;
                    }

                    let next_idx = (new_y * wi32 + new_x) as usize;
                    let next = &plant_grid[next_idx];

                    // if same plant, remove a fence from this side
                    if curr == next {
                        fence_count -= 1;
                        if !visited_vec[next_idx] {
                            visited_vec[next_idx] = true;
                            to_process.push_back((next, new_x, new_y));
                        }
                    }
                }

                // put current "group" into the map for current char
                plant_group_map
                    .get_mut(curr)
                    .unwrap()
                    .last_mut()
                    .unwrap()
                    .push(fence_count);
            }
        }
    }

    // calculate area + perimeter
    let mut result = 0;
    for plant in plant_group_map.keys() {
        let groups = plant_group_map.get(plant).unwrap();
        for group in groups {
            let area = group.len();
            let perimeter: usize = group.iter().sum();
            result += area * perimeter;
        }
    }
    println!("{}", result);
}
