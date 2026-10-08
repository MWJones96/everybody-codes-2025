use std::fs;

pub fn run(quest: &str, parts: [fn(&str) -> String; 3]) {
    for (i, part) in parts.iter().enumerate() {
        let path = format!("inputs/{quest}/p{}.txt", i + 1);
        if let Ok(input) = fs::read_to_string(&path) {
            println!("part {}: {}", i + 1, part(input.trim_end()));
        }
    }
}
