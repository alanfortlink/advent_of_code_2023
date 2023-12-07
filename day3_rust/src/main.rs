use std::{
    collections::{HashMap, HashSet},
    env,
    fs::File,
    io::{BufRead, BufReader},
    process::exit,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: ./main file_path");
        exit(1);
    }

    let mut relevants: HashSet<(i32, i32)> = HashSet::new();
    let mut relevant_source: HashMap<(i32, i32), (usize, usize)> = HashMap::new();
    let mut symbols: HashMap<(usize, usize), char> = HashMap::new();
    let mut words: HashMap<(usize, usize), String> = HashMap::new();

    let rows = BufReader::new(File::open(args[1].as_str()).unwrap()).lines();

    for (i, row) in rows.enumerate() {
        let mut word: String = String::from("");
        let mut wi: usize = 0;
        let mut wj: usize = 0;

        for (j, c) in row.unwrap().chars().enumerate() {
            if c != '.' && !c.is_ascii_digit() {
                let neigh: Vec<(i32, i32)> = vec![
                    (1, 0),
                    (-1, 0),
                    (0, 1),
                    (0, -1),
                    (1, 1),
                    (-1, -1),
                    (1, -1),
                    (-1, 1),
                ];

                symbols.insert((i, j), c);

                for (ni, nj) in neigh {
                    let ri = i as i32 + ni;
                    let rj = j as i32 + nj;

                    relevants.insert((ri, rj));
                    relevant_source.insert((ri, rj), (i, j));
                }
            }

            if c.is_ascii_digit() {
                if word.is_empty() {
                    wi = i;
                    wj = j;
                }
                word.push(c);
            } else {
                if !word.is_empty() {
                    words.insert((wi, wj), word.clone());
                }
                word.clear();
            }
        }
        if !word.is_empty() {
            words.insert((wi, wj), word.clone());
        }
        word.clear();
    }

    let mut star_count: HashMap<(usize, usize), HashSet<String>> = HashMap::new();

    for ((wi, wj), word) in words.iter() {
        for j in 0..(word.len()) {
            let key: (i32, i32) = (wi.clone() as i32, wj.clone() as i32 + j as i32);
            if relevants.contains(&key) {
                let k = relevant_source.get(&key).unwrap();
                if symbols.get(k).unwrap_or(&'.') == &'*' {
                    if !star_count.contains_key(&k) {
                        star_count.insert(*k, HashSet::new());
                    }

                    println!("{word} {}, {}", k.0, k.1);
                    star_count.get_mut(k).unwrap().insert(word.clone());
                }
            }
        }
    }

    let total: i32 = star_count
        .into_iter()
        .filter(|(_, v)| v.len() == 2)
        .map(|(_, v)| {
            v.into_iter()
                .map(|v| v.parse::<i32>().unwrap())
                .reduce(|a, b| a * b)
                .unwrap()
        })
        .sum();

    println!("{total}");
}
