use std::{
    cmp::{self, max},
    collections::{HashMap, VecDeque},
    env,
    fs::File,
    io::{BufRead, BufReader},
};

struct Hand {
    hand: String,
    kind: i32,
    bid: i32,
}

impl Hand {
    fn new(contents: String) -> Hand {
        let (hand, bid) = contents.split_once(" ").unwrap();
        let bid = bid.parse::<i32>().unwrap();

        Hand {
            hand: hand.to_owned(),
            kind: get_kind_w(&hand.clone().to_owned()),
            bid,
        }
    }
}

fn group(hand: &String) -> HashMap<char, i32> {
    let mut m = HashMap::new();
    for c in hand.chars() {
        m.insert(c, 1 + m.get(&c).unwrap_or(&0));
    }
    m
}

fn get_kind(hand: &String) -> i32 {
    for (i, clsf) in [
        |v| group(v).len() == 1,                    // five of a kind
        |v| group(v).values().max().unwrap() == &4, // four of a kind
        |v| group(v).len() == 2,                    // full house
        |v| group(v).values().max().unwrap() == &3, // three of a kind
        |v| group(v).len() == 3,                    // two pair
        |v| group(v).values().max().unwrap() == &2, // one pair
    ]
    .iter()
    .enumerate()
    {
        if clsf(&hand) {
            return 100 - i as i32;
        }
    }

    1 // highest
}

fn get_kind_w(hand: &String) -> i32 {
    let mut m = 0;

    let mut q: VecDeque<String> = VecDeque::new();
    q.push_back("".to_string());

    let v = vec!['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];

    while !q.is_empty() {
        if m == 100 {
            break;
        }

        let cand = q.pop_front().unwrap();
        if cand.len() == 5 {
            m = max(get_kind(&cand), m);
            continue;
        }

        let c = hand.chars().nth(cand.len()).unwrap();

        if c != 'J' {
            q.push_back(format!("{cand}{c}"));
            continue;
        }

        for c in v.iter() {
            q.push_back(format!("{cand}{c}"));
        }
    }

    m
}

fn main() {
    let file_path = env::args().nth(1).unwrap();
    let lines = BufReader::new(File::open(file_path.as_str()).unwrap()).lines();

    let mut hands: Vec<Hand> = lines
        .filter_map(|l| match l {
            Ok(contents) => Some(Hand::new(contents)),
            _ => None,
        })
        .collect();

    let value: HashMap<char, i32> = HashMap::from_iter(
        vec![
            'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
        ]
        .into_iter()
        .enumerate()
        .map(|(i, c)| (c, 30 - i as i32)),
    );

    hands.sort_by(|lh, rh| {
        match lh.kind.cmp(&rh.kind) {
            cmp::Ordering::Equal => (),
            v => return v,
        }

        for i in 0..5 {
            let lc = lh.hand.chars().nth(i).unwrap();
            let rc = rh.hand.chars().nth(i).unwrap();

            match value[&lc].cmp(&value[&rc]) {
                cmp::Ordering::Equal => (),
                v => return v,
            }
        }

        cmp::Ordering::Equal
    });

    let res: i32 = hands
        .into_iter()
        .enumerate()
        .map(|(i, v)| v.bid * (1 + i as i32))
        .sum();
    println!("{res}");
}
