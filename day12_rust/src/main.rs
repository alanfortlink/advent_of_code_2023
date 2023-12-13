use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file_path = env::args().nth(1).unwrap();
    let lines: Vec<String> = BufReader::new(File::open(file_path.as_str()).unwrap())
        .lines()
        .filter_map(|v| v.ok())
        .collect();

    let res: i64 = lines.into_iter().map(|l| q12(l)).sum();
    println!("{res}");
}

fn rec(
    m: &str,
    s: &[i64],
    n: bool,
    current: String,
    cache: &mut HashMap<(String, Vec<i64>, bool), i64>,
) -> i64 {
    let key = (m.to_string(), s.clone().to_vec(), n.clone());

    if cache.contains_key(&key) {
        return *cache.get(&key).unwrap();
    }

    if s.is_empty() {
        if m.contains("#") {
            return 0;
        }

        return 1;
    }

    if m.is_empty() {
        if s.iter().sum::<i64>() > 0 {
            return 0;
        }

        return 1;
    }

    let c = m.chars().nth(0).unwrap();

    if s[0] == 0 {
        if c == '?' || c == '.' {
            let r = rec(&m[1..], &s[1..], false, format!("{}{}", current, c), cache);
            cache.insert(key, r);
            return r;
        }

        return 0;
    }

    if n {
        if c == '?' || c == '#' {
            let r = rec(&m[1..], &tr(s), true, format!("{}{}", current, c), cache);
            cache.insert(key, r);
            return r;
        }

        return 0;
    }

    if c == '#' {
        let r = rec(&m[1..], &tr(s), true, format!("{}{}", current, '#'), cache);
        cache.insert(key, r);
        return r;
    }

    if c == '.' {
        let r = rec(&m[1..], s, false, format!("{}{}", current, '.'), cache);
        cache.insert(key, r);
        return r;
    }

    let r = rec(&m[1..], s, false, format!("{}{}", current, '.'), cache)
        + rec(&m[1..], &tr(s), true, format!("{}{}", current, '#'), cache);
    cache.insert(key, r);
    return r;
}

fn tr(s: &[i64]) -> Box<Vec<i64>> {
    let mut r = Box::new(Vec::new());

    r.push(s[0] - 1);
    for v in &s[1..] {
        r.push(*v);
    }

    r
}

fn q12(l: String) -> i64 {
    let (m, s) = l.split_once(" ").unwrap();
    let s = s
        .split(",")
        .map(|v| v.parse::<_>().unwrap())
        .collect::<Vec<i64>>();

    let mut m = m.to_string();
    let mut s = s;

    let om = m.clone();
    let os = s.clone();

    for _ in 0..4 {
        m.push_str(&format!("?{}", om));
        s.extend(&os);
    }

    let current = "".to_string();

    let mut cache: HashMap<(String, Vec<i64>, bool), i64> = HashMap::new();
    return rec(&format!("{}", m), &s, false, current, &mut cache);
}
