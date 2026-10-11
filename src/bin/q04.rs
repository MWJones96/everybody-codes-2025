fn part1(input: &str) -> String {
    let lines: Vec<u32> = input.lines().map(|x| x.parse::<u32>().unwrap()).collect();

    let start_gear = lines[0];
    let end_gear = lines[lines.len() - 1];

    ((start_gear * 2025) / end_gear).to_string()
}

fn part2(input: &str) -> String {
    const LAST_WHEEL_TURNS: u64 = 10_000_000_000_000;

    let lines: Vec<u64> = input.lines().map(|x| x.parse::<u64>().unwrap()).collect();

    let start_gear = lines[0];
    let end_gear = lines[lines.len() - 1];
    let first_gear_turns: f64 = (LAST_WHEEL_TURNS * end_gear) as f64 / start_gear as f64;
    first_gear_turns.ceil().to_string()
}

fn part3(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();

    let mut clicks: u64 = 100 * lines[0].parse::<u64>().unwrap();
    for gear in &lines[1..lines.len() - 1] {
        let spl: Vec<u64> = gear.split("|").map(|x| x.parse::<u64>().unwrap()).collect();
        let ratio = spl[1] / spl[0];
        clicks *= ratio;
    }

    (clicks / lines[lines.len() - 1].parse::<u64>().unwrap()).to_string()
}

fn main() {
    everybody_codes_2025::run(env!("CARGO_BIN_NAME"), [part1, part2, part3]);
}
