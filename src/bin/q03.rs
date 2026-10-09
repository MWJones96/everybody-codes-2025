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
    nums.sort();

    nums[..20].iter().sum::<u32>().to_string()
}

fn part3(input: &str) -> String {
    let nums: Vec<u32> = input.split(',').map(|x| x.parse().unwrap()).collect();

    let mut max = 0;

    let mut freq: HashMap<u32, u32> = HashMap::new();
    for num in nums {
        if !freq.contains_key(&num) {
            freq.insert(num, 0);
        }

        let new_val = freq.get(&num).unwrap() + 1;
        freq.insert(num, new_val);
        max = max.max(new_val);
    }

    max.to_string()
}

fn main() {
    everybody_codes_2025::run(env!("CARGO_BIN_NAME"), [part1, part2, part3]);
}
