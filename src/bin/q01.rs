fn part1(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let names: Vec<&str> = lines[0].split(',').collect();
    let moves: Vec<&str> = lines[2].split(',').collect();

    let mut pos: usize = 0;

    for mv in moves {
        match mv.chars().nth(0).unwrap() {
            'L' => {
                let steps: usize = mv[1..].parse().unwrap();
                if steps > pos {
                    pos = 0;
                } else {
                    pos -= steps;
                }
                pos = pos.min(names.len() - 1);
            }
            'R' => {
                let steps: usize = mv[1..].parse().unwrap();
                pos += steps;
                pos = pos.min(names.len() - 1);
            }
            _ => panic!("Invalid direction"),
        }
    }

    names[pos].to_string()
}

fn part2(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let names: Vec<&str> = lines[0].split(',').collect();
    let moves: Vec<&str> = lines[2].split(',').collect();

    let mut pos: usize = 0;

    for mv in moves {
        match mv.chars().nth(0).unwrap() {
            'L' => {
                let steps: usize = mv[1..].parse().unwrap();
                let steps: usize = steps % names.len();
                let steps = names.len() - steps;
                pos = (pos + steps) % names.len();
            }
            'R' => {
                let steps: usize = mv[1..].parse().unwrap();
                let steps: usize = steps % names.len();
                pos = (pos + steps) % names.len();
            }
            _ => panic!("Invalid direction"),
        }
    }

    names[pos].to_string()
}

fn part3(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let mut names: Vec<&str> = lines[0].split(',').collect();
    let moves: Vec<&str> = lines[2].split(',').collect();

    for mv in moves {
        match mv.chars().nth(0).unwrap() {
            'L' => {
                let steps: usize = mv[1..].parse().unwrap();
                let steps: usize = steps % names.len();
                let steps: usize = names.len() - steps;
                let j: usize = steps % names.len();
                names.swap(0, j);
            }
            'R' => {
                let steps: usize = mv[1..].parse().unwrap();
                let steps: usize = steps % names.len();
                let j: usize = steps % names.len();
                names.swap(0, j);
            }
            _ => panic!("Invalid direction"),
        }
    }

    names[0].to_string()
}

fn main() {
    everybody_codes_2025::run(env!("CARGO_BIN_NAME"), [part1, part2, part3]);
}
