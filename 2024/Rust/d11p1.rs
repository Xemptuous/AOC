fn main() {
    let string = std::fs::read_to_string("inputs/d11.in").expect("Couldn't read file");
    let mut numbers: Vec<String> = string.split_whitespace().map(String::from).collect();
    numbers.reserve(1000000);

    for _ in 0..25 {
        // 25 blinks
        let mut i = 0;
        while i < numbers.len() {
            let n = numbers[i].clone();
            // Rule 1: if 0, then 1
            if n == "0" {
                numbers[i].remove(0);
                numbers[i].insert(0, '1');
                i += 1;
            }
            // Rule 2: if even number, split into 2. No leading zeros (for right)
            else if n.len() % 2 == 0 {
                let (left, right) = n.split_at(n.len() / 2);
                // replace left number
                numbers[i] = String::from(left);
                if right.starts_with('0') {
                    if !right.chars().all(|c| c == '0') {
                        numbers.insert(i + 1, String::from(right.trim_start_matches('0')));
                    } else {
                        numbers.insert(i + 1, String::from("0"));
                    }
                } else {
                    numbers.insert(i + 1, String::from(right));
                }
                i += 2;
            }
            // Rule 3: else, * 2024
            else {
                let num = n.parse::<u64>().unwrap();
                numbers[i] = (num * 2024).to_string();
                i += 1;
            }
        }
    }
    println!("{}", numbers.len());
}
