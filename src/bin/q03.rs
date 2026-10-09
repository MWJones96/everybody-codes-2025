use std::collections::{HashMap, HashSet};

fn part1(input: &str) -> String {
    let nums: Vec<u32> = input.split(',').map(|x| x.parse().unwrap()).collect();
    let nums: HashSet<u32> = HashSet::from_iter(nums);

    nums.iter().sum::<u32>().to_string()
}

fn part2(input: &str) -> String {
    let nums: Vec<u32> = input.split(',').map(|x| x.parse().unwrap()).collect();
    let nums: HashSet<u32> = HashSet::from_iter(nums);
    let mut nums: Vec<u32> = nums.into_iter().collect();
    nums.sort_unstable();

    nums[..20].iter().sum::<u32>().to_string()
}

fn part3(input: &str) -> String {
    let nums: Vec<u32> = input.split(',').map(|x| x.parse().unwrap()).collect();

    let mut freq: HashMap<u32, u32> = HashMap::new();
    for num in nums {
        *freq.entry(num).or_insert(0) += 1;
    }

    freq.values().max().unwrap_or(&0).to_string()
}

fn main() {
    everybody_codes_2025::run(env!("CARGO_BIN_NAME"), [part1, part2, part3]);
}
