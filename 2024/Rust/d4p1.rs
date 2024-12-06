const W: usize = 140;
const H: usize = 140;

const FORWARD: &str = "XMAS";
const BACKWARD: &str = "SAMX";

fn xy_idx(x: usize, y: usize) -> usize {
    x * W + y
}

fn horizontal(x: usize, y: usize, input: &[u8]) -> i32 {
    if y >= W - 3 {
        return 0;
    }
    let i = xy_idx(x, y);
    let word = match input.get(i..i + 4) {
        Some(x) => String::from_utf8_lossy(x),
        None => return 0,
    };
    if word == FORWARD || word == BACKWARD {
        return 1;
    }
    return 0;
}

fn vertical(x: usize, y: usize, input: &[u8]) -> i32 {
    if x >= H - 3 {
        return 0;
    }
    let arr: [u8; 4] = [
        input[xy_idx(x, y)],
        input[xy_idx(x + 1, y)],
        input[xy_idx(x + 2, y)],
        input[xy_idx(x + 3, y)],
    ];
    let word = String::from_utf8_lossy(&arr);
    if word == FORWARD || word == BACKWARD {
        return 1;
    }
    return 0;
}

fn south_east(x: usize, y: usize, input: &[u8]) -> i32 {
    let mut count = 0;
    let arr: [u8; 4] = [
        input[xy_idx(x, y)],
        input[xy_idx(x + 1, y + 1)],
        input[xy_idx(x + 2, y + 2)],
        input[xy_idx(x + 3, y + 3)],
    ];
    let word = String::from_utf8_lossy(&arr);
    if word == FORWARD {
        count += 1
    }
    if word == BACKWARD {
        count += 1
    }
    count
}

fn south_west(x: usize, y: usize, input: &[u8]) -> i32 {
    let mut count = 0;
    let arr: [u8; 4] = [
        input[xy_idx(x, y)],
        input[xy_idx(x + 1, y - 1)],
        input[xy_idx(x + 2, y - 2)],
        input[xy_idx(x + 3, y - 3)],
    ];
    let word = String::from_utf8_lossy(&arr);
    if word == FORWARD {
        count += 1
    }
    if word == BACKWARD {
        count += 1
    }
    count
}

fn main() {
    let input: Vec<u8> = std::fs::read("input.txt")
        .expect("Couldn't open file.")
        .into_iter()
        .filter(|&b| !b.is_ascii_whitespace())
        .collect();

    let mut result = 0;
    for x in 0..W {
        for y in 0..H {
            result += horizontal(x, y, &input);
            result += vertical(x, y, &input);
            if x <= H - 4 && y <= W - 4 {
                result += south_east(x, y, &input);
            }
            if x <= H - 4 && y >= 3 {
                result += south_west(x, y, &input);
            }
        }
    }
    println!("{}", result);
}
