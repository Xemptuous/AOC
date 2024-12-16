use std::cell::Cell;

fn main() {
    let file = std::fs::read_to_string("inputs/d15.in").expect("Couldn't read file");

    let (grid, movements) = file
        .split_once("\n\n")
        .map(|(g, m)| (g.as_bytes(), m.as_bytes()))
        .unwrap();
    let (h, w) = (
        grid.iter().filter(|&c| c.is_ascii_whitespace()).count(),
        grid.iter().position(|&c| c.is_ascii_whitespace()).unwrap() + 1,
    );
    let (player, mut tiles) = generate_map(grid, w);
    for movement in movements {
        match movement {
            b'<' => try_move_player(&player, &mut tiles, w, -1, 0),
            b'>' => try_move_player(&player, &mut tiles, w, 1, 0),
            b'^' => try_move_player(&player, &mut tiles, w, 0, -1),
            b'v' => try_move_player(&player, &mut tiles, w, 0, 1),
            _ => {},
        }
    }

    let mut result = 0;
    // calculate scores
    for y in 0..h {
        for x in 0..w {
            if tiles[y * w + x] == Tile::Box {
                result += 100 * y + x;
            }
        }
    }
    println!("{}", result);
}

#[derive(Clone, PartialEq)]
enum Tile {
    Wall,
    Floor,
    Box,
    None,
}

#[derive(Default)]
struct Player {
    x: Cell<i32>,
    y: Cell<i32>,
}

fn try_move_player(player: &Player, tiles: &mut [Tile], width: usize, dx: i32, dy: i32) {
    let (new_x, new_y) = (player.x.get() + dx, player.y.get() + dy);
    let new_idx = (new_y * width as i32 + new_x) as usize;
    if let Some(tile) = tiles.get(new_idx) {
        use Tile::*;
        match tile {
            Wall => (),
            Floor => {
                player.x.set(new_x);
                player.y.set(new_y);
            },
            Box => {
                if try_move_boxes(tiles, width, new_x, new_y, dx, dy) {
                    player.x.set(new_x);
                    player.y.set(new_y);
                }
            },
            _ => {},
        }
    }
}

fn try_move_boxes(tiles: &mut [Tile], width: usize, x: i32, y: i32, dx: i32, dy: i32) -> bool {
    use Tile::*;
    let (into_x, into_y) = (x + dx, y + dy);
    let wi32 = width as i32;
    let curr_idx = (y * wi32 + x) as usize;
    let into_idx = (into_y * wi32 + into_x) as usize;
    if let Some(into_tile) = tiles.get(into_idx) {
        return match into_tile {
            Wall => false,
            Floor => {
                tiles.swap(curr_idx, into_idx);
                true
            },
            Box => {
                if try_move_boxes(tiles, width, into_x, into_y, dx, dy) {
                    tiles.swap(curr_idx, into_idx);
                    return true;
                }
                false
            },
            _ => false,
        };
    }
    false
}

fn generate_map(grid: &[u8], w: usize) -> (Player, Vec<Tile>) {
    let player = Player::default();
    let mut tiles: Vec<Tile> = vec![Tile::None; grid.len()];

    let wi32 = w as i32;
    for (i, b) in grid.iter().enumerate() {
        let ii32 = i as i32;
        let (x, y) = (ii32 % wi32, ii32 / wi32);
        match b {
            b'#' => tiles[i] = Tile::Wall,
            b'O' => tiles[i] = Tile::Box,
            b'@' => {
                tiles[i] = Tile::Floor;
                player.x.set(x);
                player.y.set(y);
            },
            b'.' => tiles[i] = Tile::Floor,
            _ => {},
        }
    }
    (player, tiles)
}
