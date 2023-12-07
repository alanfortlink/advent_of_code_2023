use std::{
    env,
    fs::File,
    io::{BufRead, BufReader, Lines},
};

fn main() {
    let file_path = env::args().nth(1).unwrap();
    let mut lines = BufReader::new(File::open(file_path.as_str()).unwrap()).lines();

    let helper = |v: &mut Lines<BufReader<File>>| {
        v.next()
            .unwrap()
            .unwrap()
            .split_once(":")
            .unwrap()
            .1
            .replace(" ", "")
            .parse::<i64>()
            .unwrap()
    };

    let t = helper(&mut lines);
    let d = helper(&mut lines);

    println!("{}", (0..t).filter(|i| (t - i) * i > d).count());
}
