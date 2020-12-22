use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

type Map = Vec<Vec<u8>>;

fn run_algorithm(map: &Map, (r_step, d_step): (usize, usize)) -> usize {
    let mut right = 0;
    let mut down = 0;
    let mut tree_count = 0;
    while down < map.len() {
        tree_count += (map[down][right % map[down].len()] == b'#') as usize;
        right += r_step;
        down += d_step;
    }
    tree_count
}

const ALGORITHMS: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

fn main() -> io::Result<()> {
    let map = BufReader::new(File::open("input")?)
        .lines()
        .filter_map(Result::ok)
        .map(String::into_bytes)
        .collect();

    println!(
        "{}",
        ALGORITHMS
            .iter()
            .map(|alg| run_algorithm(&map, *alg))
            .fold(1, |x, acc| x * acc)
    );
    Ok(())
}
