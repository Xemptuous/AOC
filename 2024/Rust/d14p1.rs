use std::cell::Cell;

const W: i32 = 101;
const H: i32 = 103;

struct Robot {
    x: Cell<i32>,
    y: Cell<i32>,
    dx: Cell<i32>,
    dy: Cell<i32>,
}

impl Robot {
    pub fn try_move(&self) {
        let mut new_x = self.x.get() + self.dx.get();
        let mut new_y = self.y.get() + self.dy.get();

        // if moving left, teleport right
        if new_x < 0 {
            new_x += W;
        }
        // if moving right, teleport left
        else if new_x >= W {
            new_x -= W;
        }

        // if moving up, teleport down
        if new_y < 0 {
            new_y += H;
        }
        // if moving down, teleport up
        else if new_y >= H {
            new_y -= H;
        }

        self.x.set(new_x);
        self.y.set(new_y);
    }
}

fn parse_to_robot(line: &str) -> Robot {
    let parts: Vec<&str> = line.split(' ').collect();
    let coordinates: Vec<i32> = parts[0]
        .trim_start_matches("p=")
        .split(',')
        .map(|c| c.parse::<i32>().unwrap())
        .collect();
    let velocities: Vec<i32> = parts[1]
        .trim_start_matches("v=")
        .split(',')
        .map(|c| c.parse::<i32>().unwrap())
        .collect();

    Robot {
        x: Cell::new(coordinates[0]),
        y: Cell::new(coordinates[1]),
        dx: Cell::new(velocities[0]),
        dy: Cell::new(velocities[1]),
    }
}

fn main() {
    let file_string = std::fs::read_to_string("inputs/d14.in").expect("Couldn't read file");
    let lines = file_string.lines();

    // Quadrants
    // ---------
    // | 1 | 2 |
    // ---------
    // | 3 | 4 |
    // ---------
    let mut quadrants: Vec<i32> = vec![0; 4];
    let mw = (W - 1) / 2;
    let mh = (H - 1) / 2;

    for line in lines {
        let robot = parse_to_robot(line);

        // move robots 100 times
        for _ in 0..100 {
            robot.try_move();
        }

        // check where they end up
        let (x, y) = (robot.x.get(), robot.y.get());
        // Q1
        if (0..mw).contains(&x) && (0..mh).contains(&y) {
            quadrants[0] += 1;
        }
        // Q2
        else if (mw + 1..W).contains(&x) && (0..mh).contains(&y) {
            quadrants[1] += 1;
        }
        // Q3
        else if (0..mw).contains(&x) && (mh + 1..H).contains(&y) {
            quadrants[2] += 1;
        }
        // Q4
        else if (mw + 1..W).contains(&x) && (mh + 1..H).contains(&y) {
            quadrants[3] += 1;
        }
    }
    println!("{}", quadrants.iter().product::<i32>());
}
