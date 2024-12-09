#!/usr/bin/env rust-script

struct Block {
    id: usize,
    empty: bool,
}

fn main() {
    let disk_map = std::fs::read("inputs/d9.in").expect("Couldn't open file");

    // create initial checksum with blocks
    let mut id: usize = 0;
    let mut filesystem: Vec<Block> = Vec::with_capacity(disk_map.len() * 10);
    for block in disk_map.chunks(2) {
        if let &[a, b] = block {
            for _ in 0..u32::from(a - 48) {
                filesystem.push(Block { id, empty: false });
            }
            id += 1;
            for _ in 0..u32::from(b - 48) {
                filesystem.push(Block { id: 0, empty: true });
            }
        }
    }
    // if odd, add last item
    if disk_map.len() % 2 == 1 {
        if let Some(a) = disk_map.last() {
            for _ in 0..u32::from(a - 48) {
                filesystem.push(Block { id, empty: false });
            }
        }
    }

    let n = filesystem.len() - 1;
    let mut end_ptr = n;
    // get first empty index
    let mut empty_ptr = filesystem.iter().position(|block| block.empty).unwrap();

    // reorder blocks until contiguous
    while empty_ptr < end_ptr && end_ptr > 0 && empty_ptr < n {
        if filesystem[empty_ptr].empty && !filesystem[end_ptr].empty {
            filesystem.swap(empty_ptr, end_ptr);
            end_ptr -= 1;
            empty_ptr += 1;
        } else if !filesystem[empty_ptr].empty {
            empty_ptr += 1;
        } else if filesystem[end_ptr].empty {
            end_ptr -= 1;
        }
    }

    // calculate checksum
    let mut checksum: usize = 0;
    for (i, block) in filesystem.iter().enumerate() {
        checksum += i * block.id;
    }
    println!("{}", checksum);
}
