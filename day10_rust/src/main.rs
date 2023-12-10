use std::{
    collections::HashSet,
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn out_of_bounds(grid: &Vec<Vec<char>>, i: i32, j: i32) -> bool {
    i < 0 || j < 0 || i >= grid.len() as i32 || j >= grid[i as usize].len() as i32
}

fn contains(grid: &Vec<Vec<char>>, i: i32, j: i32, items: &str) -> bool {
    !out_of_bounds(grid, i, j) && items.contains(grid[i as usize][j as usize])
}

fn get_s_dir(grid: &Vec<Vec<char>>, i: i32, j: i32) -> char {
    let north = contains(grid, i - 1, j, "|7F");
    let south = contains(grid, i + 1, j, "|LJ");
    let west = contains(grid, i, j - 1, "-LF");
    let east = contains(grid, i, j + 1, "-J7");

    if north && south {
        return '|';
    }

    if east && west {
        return '-';
    }

    if north && east {
        return 'L';
    }

    if north && west {
        return 'J';
    }

    if south && west {
        return '7';
    }

    if south && east {
        return 'F';
    }

    '.'
}

fn find_path(
    grid: &Vec<Vec<char>>,
    i: i32,
    j: i32,
    seen: &mut HashSet<(i32, i32)>,
    origin: (i32, i32),
    path: &mut Vec<(i32, i32)>,
) -> i32 {
    if out_of_bounds(grid, i, j) {
        return -1;
    }

    let mut c = grid[i as usize][j as usize];

    if seen.contains(&(i, j)) {
        return match c {
            'S' => 0,
            _ => -1,
        };
    }

    seen.insert((i, j));

    if c == 'S' {
        c = get_s_dir(grid, i, j);
    }

    let directions: Vec<(i32, i32)> = match c {
        '|' => vec![(-1, 0), (1, 0)],
        '-' => vec![(0, -1), (0, 1)],
        'L' => vec![(-1, 0), (0, 1)],
        'J' => vec![(-1, 0), (0, -1)],
        '7' => vec![(0, -1), (1, 0)],
        'F' => vec![(0, 1), (1, 0)],
        _ => vec![],
    }
    .into_iter()
    .filter(|d| (i + d.0, j + d.1) != origin)
    .collect();

    path.push((i, j));
    for d in directions {
        match find_path(grid, i + d.0, j + d.1, seen, (i, j), path) {
            -1 => (),
            v => return v,
        }
    }
    path.pop();

    return -1;
}

fn loop_size(grid: &Vec<Vec<char>>) -> (i32, Vec<(i32, i32)>, (i32, i32)) {
    let mut seen: HashSet<(i32, i32)> = HashSet::new();
    let mut path: Vec<(i32, i32)> = Vec::new();

    let (i, j) = grid
        .iter()
        .enumerate()
        .filter_map(
            |(i, r)| match r.iter().enumerate().filter(|(_, c)| *c == &'S').nth(0) {
                Some(v) => Some((i, v.0)),
                _ => None,
            },
        )
        .nth(0)
        .unwrap();

    return (
        find_path(grid, i as i32, j as i32, &mut seen, (-1, -1), &mut path),
        path,
        (i as i32, j as i32),
    );
}

fn main() {
    let file_path = env::args().nth(1).unwrap();
    let lines = BufReader::new(File::open(file_path.as_str()).unwrap()).lines();
    let mut grid: Vec<Vec<char>> = lines
        .filter_map(|v| match v {
            Ok(contents) => {
                if contents.is_empty() {
                    None
                } else {
                    Some(contents)
                }
            }
            Err(_) => None,
        })
        .map(|v| v.chars().collect::<Vec<char>>())
        .collect();

    let (_, path, (si, sj)) = loop_size(&grid);

    let walls: HashSet<(i32, i32)> = HashSet::from_iter(path.into_iter());
    let mut seen: HashSet<(i32, i32)> = HashSet::new();

    grid[si as usize][sj as usize] = get_s_dir(&grid, si, sj);

    let mut count: i32 = 0;
    for i in 0..grid.len() {
        let mut inside = false;
        for j in 0..grid[i].len() {
            let p = (i as i32, j as i32);
            if !walls.contains(&p) && inside {
                seen.insert(p);
                count += 1;
            } else if walls.contains(&p) && is_vertical(grid[i][j]) {
                inside = !inside;
            }
        }
    }

    println!("{count}");
}

fn is_vertical(j: char) -> bool {
    "LJ|".contains(j)
}
