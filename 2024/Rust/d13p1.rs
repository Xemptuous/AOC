fn parse_line(line: &str, prefix: &str) -> (u64, u64) {
    let data = line.trim_start_matches(prefix).trim();
    let parts: Vec<&str> = data.split(',').collect();

    let x: u64 = parts[0]
        .trim_start_matches("X+")
        .trim_start_matches("X=")
        .parse::<u64>()
        .expect("Invalid X");
    let y: u64 = parts[1]
        .trim()
        .trim_start_matches("Y+")
        .trim_start_matches("Y=")
        .parse::<u64>()
        .expect("Invalid Y");

    (x, y)
}

fn main() {
    let file_string = std::fs::read_to_string("inputs/d13.in").expect("Couldn't open file");
    let mut lines = file_string.lines();

    let mut total_tokens = 0;
    while let Some(line) = lines.next() {
        if line.starts_with("Button A:") {
            let button_a = parse_line(line, "Button A: ");
            let button_b = parse_line(lines.next().unwrap(), "Button B: ");
            let prize = parse_line(lines.next().unwrap(), "Prize: ");

            let mut token_cost = u64::MAX;
            for a_press in 0..prize.0 {
                let (ax, ay) = (a_press * button_a.0, a_press * button_a.1);
                for b_press in 0..prize.1 {
                    let (bx, by) = (b_press * button_b.0, b_press * button_b.1);
                    let res_x = ax + bx;
                    let res_y = ay + by;
                    // if either larger than possible, break to next #A presses
                    if res_x > prize.0 || res_y > prize.1 {
                        break;
                    }
                    if res_x == prize.0 && res_y == prize.1 {
                        // get the minimum possible tokens used
                        // 3 == cost of A press
                        token_cost = token_cost.min(a_press * 3 + b_press);
                    }
                }
            }
            // if an option that leads to the prize
            if token_cost != u64::MAX {
                total_tokens += token_cost;
            }
        }
    }
    println!("{}", total_tokens);
}
