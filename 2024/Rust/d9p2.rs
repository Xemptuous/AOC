#!/usr/bin/env rust-script

#[derive(Debug)]
struct Block {
    id: usize,
    empty: bool,
}

fn main() {
    let disk_map = std::fs::read("inputs/d9.in").expect("Couldn't open file");

    // create initial filesystem with blocks
    let mut filesystem = read_filesystem(&disk_map);

    let mut file_start = 0; // to keep track of later iterations
    let mut file_end = filesystem.len() - 1;
    let mut empty_start = 0;

    // reorder blocks until contiguous (from end to start)
    while file_end > 0 {
        if empty_start >= file_end {
            if file_end == 0 {
                break;
            }
            file_end = file_start - 1;
            file_start = file_end;
            empty_start = 0;
        }
        if filesystem[empty_start].empty && !filesystem[file_end].empty {
            // get the file's range
            file_start = find_file_start(&filesystem, file_end);
            let file_range = &filesystem[file_start..=file_end];
            let file_length = file_range.len();

            // get the empty range
            let empty_end = find_empty_end(&filesystem, empty_start);
            let empty_range = &filesystem[empty_start..empty_end];
            let empty_length = empty_range.len();

            // if it fitteth, so shall it be put
            if file_length <= empty_length {
                swap_slices(&mut filesystem, empty_start, file_start, file_length);
                file_end = file_start; // move past attempted ordered file id block slice
                empty_start = 0; // reset to start
            } else {
                empty_start = empty_end;
            }
        } else if !filesystem[empty_start].empty {
            empty_start += 1;
        } else if filesystem[file_end].empty {
            file_end -= 1;
        }
    }

    // calculate checksum
    let mut checksum: usize = 0;
    for (i, block) in filesystem.iter().enumerate() {
        checksum += i * block.id;
    }
    println!("{}", checksum);
}

fn read_filesystem(disk_map: &[u8]) -> Vec<Block> {
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
    filesystem
}

fn find_file_start(filesystem: &[Block], file_end: usize) -> usize {
    let mut file_start = file_end;
    while filesystem[file_end].id == filesystem[file_start - 1].id && !filesystem[file_start].empty
    {
        file_start -= 1;
    }
    file_start
}

fn find_empty_end(filesystem: &[Block], empty_start: usize) -> usize {
    let mut empty_end = empty_start;
    while filesystem[empty_end].empty {
        empty_end += 1;
    }
    empty_end
}

fn swap_slices(
    filesystem: &mut [Block],
    empty_start: usize,
    file_start: usize,
    file_length: usize,
) {
    let (left, right) = filesystem.split_at_mut(file_start);
    let left_slice = &mut left[empty_start..empty_start + file_length];
    let right_slice = &mut right[..file_length];
    left_slice.swap_with_slice(right_slice);
}

// fn print_fs(filesystem: &[Block]) {
//     for i in filesystem.iter() {
//         if i.empty {
//             print!(".");
//         } else {
//             print!("{}", i.id)
//         }
//     }
//     println!();
// }
