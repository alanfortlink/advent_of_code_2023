use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    process::exit,
};

fn get(line: &String, iter: impl Iterator<Item = usize>) -> i32 {
    let val: Vec<&str> = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    for i in iter {
        for j in 0..val.len() {
            if line[i..].starts_with(val[j]) {
                return j as i32 + 1;
            }
        }
        let c = line.chars().nth(i).unwrap();
        if c.is_ascii_digit() {
            return c.to_digit(10).unwrap() as i32;
        }
    }
    0
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: ./main file_path");
        exit(1);
    }
    let total: i32 = BufReader::new(File::open(args[1].as_str()).unwrap())
        .lines()
        .filter_map(|l| match l {
            Ok(c) => Some(10 * get(&c, 0..c.len()) + get(&c, (0..c.len()).rev())),
            _ => None,
        })
        .sum();
    println!("{total}");
}
