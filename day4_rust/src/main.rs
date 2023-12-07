use std::{
    collections::{HashMap, HashSet},
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file_path = env::args().nth(1).unwrap();
    let mut extra: HashMap<usize, i32> = HashMap::new();
    let helper =
        |v: &str| -> HashSet<i32> { v.split_whitespace().map(|x| x.parse().unwrap()).collect() };

    let total: i32 = BufReader::new(File::open(file_path).unwrap())
        .lines()
        .enumerate()
        .map(|(index, f)| match f {
            Ok(contents) => {
                let (win_side, owned_side) = contents.split_once(" | ").unwrap();
                let win_numbers = helper(win_side.split_once(": ").unwrap().1);
                let owned_numbers = helper(owned_side);

                let intersecting = win_numbers.intersection(&owned_numbers).count();
                let num_cards = extra.get(&index).unwrap_or(&0) + 1;

                for i in 0..(intersecting) {
                    let receiver = index + i + 1;
                    let current = extra.get(&receiver).unwrap_or(&0);
                    extra.insert(receiver, current + num_cards);
                }

                num_cards
            }
            _ => 0,
        })
        .sum();
    println!("{total}");
}
