use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn solve(contents: &String) -> i32 {
    let mut rounds: Vec<Vec<i32>> = Vec::new();
    rounds.push(
        contents
            .split_whitespace()
            .map(|v| v.parse::<i32>().unwrap())
            .collect(),
    );

    while rounds.last().unwrap().iter().any(|v| v != &0) {
        let last_round = rounds.last().unwrap();
        rounds.push(
            (0..last_round.len() - 1)
                .map(|i| last_round[i + 1] - last_round[i])
                .collect::<Vec<i32>>(),
        );
    }

    let mut current = 0;
    for i in (0..rounds.len() - 1).rev() {
        current = rounds[i].first().unwrap() - current;
    }

    return current;
}

fn main() {
    let file_path = env::args().nth(1).unwrap();
    let total: i32 = BufReader::new(File::open(file_path.as_str()).unwrap())
        .lines()
        .filter_map(|v| match v {
            Ok(contents) => Some(solve(&contents)),
            _ => None,
        })
        .sum();
    println!("{total}");
}
