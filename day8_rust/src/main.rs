use std::{
    cmp::{max, min},
    collections::HashMap,
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn solve(m: &HashMap<String, (String, String)>, origin: &String, dir: &String, i: usize) -> i64 {
    if origin.chars().last().unwrap() == 'Z' {
        return 0;
    }

    if dir.chars().nth(i).unwrap() == 'L' {
        return 1 + solve(m, &m[origin].0, dir, (i + 1) % dir.len());
    }

    return 1 + solve(m, &m[origin].1, dir, (i + 1) % dir.len());
}

fn main() {
    let file_path = env::args().nth(1).unwrap();
    let mut lines = BufReader::new(File::open(file_path.as_str()).unwrap())
        .lines()
        .filter_map(|v| match v {
            Ok(line) => Some(line),
            _ => None,
        })
        .filter(|l| !l.is_empty());

    let dir = lines.next().unwrap();
    let m: HashMap<String, (String, String)> = lines
        .map(|l| {
            let (k, v) = l.split_once(" = ").unwrap();
            let v = v.replace("(", "").replace(")", "");
            let (l, r) = v.split_once(", ").unwrap();
            (String::from(k), (String::from(l), String::from(r)))
        })
        .collect();

    let values: Vec<i64> = m
        .keys()
        .filter(|k| k.chars().last().unwrap() == 'A')
        .map(|k| solve(&m, &k, &dir, 0))
        .collect();

    let gcd = |_a, _b| {
        let mut min = min(_a, _b);
        let mut max = max(_a, _b);

        while (max % min) != 0 {
            let res = max % min;
            max = min;
            min = res;
        }
        min
    };

    let lcm = |a, b| a * b / gcd(a, b);
    println!("{}", values.into_iter().reduce(lcm).unwrap());
}
