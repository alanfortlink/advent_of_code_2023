use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file_path = env::args().nth(1).unwrap();
    let lines = BufReader::new(File::open(file_path.as_str()).unwrap())
        .lines()
        .filter_map(|v| v.ok());

    let mut ms: Vec<Vec<String>> = Vec::new();
    let mut res = 0;

    let mut current: Vec<String> = Vec::new();

    for line in lines {
        if line.is_empty() {
            ms.push(current.clone());
            current.clear();
            continue;
        }

        current.push(line);
    }

    if !current.is_empty() {
        ms.push(current.clone());
        current.clear();
    }

    for m in ms {
        for j in 0..m[0].len() - 1 {
            if reflected_in_column(&m, j as i32, j as i32 + 1) {
                println!("Found reflection between columns {} and {}", j, j + 1);
                res += j + 1;
                continue;
            }
        }

        for i in 0..m.len() - 1 {
            if reflected_in_row(&m, i as i32, i as i32 + 1) {
                println!("Found reflection between rows {} and {}", i, i + 1);
                res += 100 * (i + 1);
                continue;
            }
        }
    }

    println!("{res}");
}

fn reflected_in_row(m: &[String], p: i32, q: i32) -> bool {
    let mut p = p.clone();
    let mut q = q.clone();

    let mut errors = 0;

    loop {
        if p < 0 {
            return errors == 1;
        }

        if q >= m.len() as i32 {
            return errors == 1;
        }

        for j in 0..m[p as usize].len() {
            if m[p as usize].chars().nth(j) != m[q as usize].chars().nth(j) {
                errors += 1;
            }
        }

        p = p - 1;
        q = q + 1;
    }
}

fn reflected_in_column(m: &[String], p: i32, q: i32) -> bool {
    let mut p = p;
    let mut q = q;

    let mut errors = 0;

    loop {
        if p < 0 {
            return errors == 1;
        }

        if q >= m[0].len() as i32 {
            return errors == 1;
        }

        for j in 0..m.len() {
            if m[j].chars().nth(p as usize) != m[j].chars().nth(q as usize) {
                errors += 1;
            }
        }

        p = p - 1;
        q = q + 1;
    }
}
