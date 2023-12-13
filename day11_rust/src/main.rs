use std::{
    collections::HashSet,
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file_path = env::args().nth(1).unwrap();
    let lines: Vec<Vec<char>> = BufReader::new(File::open(file_path.as_str()).unwrap())
        .lines()
        .filter_map(|v| match v {
            Ok(contents) => Some(contents.chars().collect()),
            _ => None,
        })
        .collect();

    let mut rows: HashSet<usize> = HashSet::new();
    let mut cols: HashSet<usize> = HashSet::new();

    for i in 0..lines.iter().len() {
        for j in 0..lines[i].len() {
            if lines[i][j] == '#' {
                rows.insert(i);
                cols.insert(j);
            }
        }
    }

    let mut positions: Vec<(usize, usize)> = Vec::new();

    let mut iinc = 0;
    for i in 0..lines.iter().len() {
        if !rows.contains(&i) {
            iinc += 1000000 - 1;
            continue;
        }

        let mut jinc = 0;
        for j in 0..lines[i].len() {
            if !cols.contains(&j) {
                jinc += 1000000 - 1;
                continue;
            }

            if lines[i][j] == '#' {
                positions.push((i + iinc, j + jinc));
            }
        }
    }

    let mut total = 0;
    for i in 0..positions.len() - 1 {
        let (ix, iy) = positions[i];
        for j in i + 1..positions.len() {
            let (jx, jy) = positions[j];
            total += (jx as i64 - ix as i64).abs() + (jy as i64 - iy as i64).abs();
        }
    }

    println!("{total}");
}
