use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

type Map = Vec<Vec<u8>>;

struct Algorithm {
    right: usize,
    down: usize,
}

fn run_algorithm(map: &Map, alg: &Algorithm) -> usize {
    let mut right = 0;
    let mut down = 0;
    let mut tree_count = 0;
    while down < map.len() {
        tree_count += (map[down][right % map[down].len()] == b'#') as usize;
        right += alg.right;
        down += alg.down;
    }
    tree_count
}

const ALGORITHMS: [Algorithm; 5] = [
    Algorithm { right: 1, down: 1 },
    Algorithm { right: 3, down: 1 },
    Algorithm { right: 5, down: 1 },
    Algorithm { right: 7, down: 1 },
    Algorithm { right: 1, down: 2 },
];

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
            .map(|alg| run_algorithm(&map, alg))
            .fold(1, |x, acc| x * acc)
    );
    Ok(())
}
