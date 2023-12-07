use std::{
    cmp::{max, min},
    collections::{HashMap, VecDeque},
    env,
    fs::File,
    io::{BufRead, BufReader},
    ops::Range,
};

fn main() {
    let file_path = env::args().nth(1).unwrap();
    let mut lines = BufReader::new(File::open(file_path.as_str()).unwrap()).lines();
    let seeds = lines
        .next()
        .unwrap()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|v| v.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut ranges: Vec<Range<i64>> = Vec::new();

    for (i, v) in seeds.iter().enumerate().step_by(2) {
        ranges.push((*v)..(v + seeds[i + 1]));
    }

    lines.next(); // empty

    loop {
        let step = lines.next().unwrap_or(Ok(String::from(""))).unwrap();

        if step.is_empty() {
            break;
        }

        let mut level_ranges: HashMap<Range<i64>, Range<i64>> = HashMap::new();

        loop {
            let line = lines.next().unwrap_or(Ok(String::from(""))).unwrap();

            if line.is_empty() {
                break;
            }

            let info = line
                .split_whitespace()
                .map(|v| v.parse::<i64>().unwrap())
                .collect::<Vec<_>>();

            let destination = info[0];
            let origin = info[1];
            let length = info[2];

            level_ranges.insert(origin..origin + length, destination..destination + length);
        }

        let mut q: VecDeque<Range<i64>> = VecDeque::from_iter(ranges.iter().map(|v| v.clone()));
        ranges.clear();

        while !q.is_empty() {
            let item = q.pop_front().unwrap();
            q5(&item, &level_ranges, &mut ranges, &mut q);
        }
    }

    let result = ranges.into_iter().map(|v| v.start).min().unwrap();
    println!("{result}");
}

fn q5(
    item: &Range<i64>,
    level_ranges: &HashMap<Range<i64>, Range<i64>>,
    ranges: &mut Vec<Range<i64>>,
    q: &mut VecDeque<Range<i64>>,
) {
    let intersecting = level_ranges
        .iter()
        .filter(|v| !(item.start >= v.0.end || v.0.start >= item.end))
        .collect::<Vec<_>>();

    if intersecting.is_empty() {
        ranges.push(item.clone());
        return;
    }

    for (r_o, r_d) in intersecting {
        let diff = r_d.start - r_o.start;
        let before = min(r_o.start, item.start)..max(r_o.start, item.start);
        let after = min(r_o.end, item.end)..max(r_o.end, item.end);
        let inter = max(item.start, r_o.start) + diff..min(item.end, r_o.end) + diff;

        // item contained in range
        if item.start >= r_o.start && item.end <= r_o.end {
            ranges.push(item.start + diff..item.end + diff);
        }
        // range contained in item
        else if r_o.start >= item.start && r_o.end <= item.end {
            q.push_back(before);
            q.push_back(after);
            ranges.push(inter);
        }
        // range starts after item
        else if r_o.start >= item.start && r_o.start < item.end {
            q.push_back(before);
            ranges.push(inter);
        }
        // item starts after range
        else if item.start >= r_o.start && item.start < r_o.end {
            q.push_back(after);
            ranges.push(inter);
        }
    }
}
