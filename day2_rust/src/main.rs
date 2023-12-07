use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn q2(l: &str) -> i32 {
    let mut counts: HashMap<&str, i32> = HashMap::new();
    for reveal in l.split_once(": ").unwrap().1.split("; ") {
        for (amount, color) in reveal.split(", ").map(|f| f.split_once(" ").unwrap()) {
            let curr = *counts.get(color).unwrap_or(&0);
            counts.insert(color, amount.parse::<i32>().unwrap().max(curr));
        }
    }
    counts.values().fold(1, |a, b| a * b)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let total: i32 = BufReader::new(File::open(args[1].as_str()).unwrap())
        .lines()
        .filter_map(|l| match l {
            Ok(content) => Some(q2(&content)),
            Err(_) => None,
        })
        .sum();

    println!("{total}");
}



