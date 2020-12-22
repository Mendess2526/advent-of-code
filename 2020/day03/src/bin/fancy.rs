use std::{
    convert::TryFrom,
    fs::File,
    io::{self, BufRead, BufReader},
    iter::FromIterator,
    ops::Index,
    str::FromStr,
};

#[derive(Clone, Copy)]
enum Tile {
    Space = 0,
    Tree = 1,
}

impl TryFrom<char> for Tile {
    type Error = char;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '#' => Ok(Tile::Tree),
            '.' => Ok(Tile::Space),
            _ => Err(c),
        }
    }
}

struct WrappingArray<T>(Box<[T]>);

impl<T> Index<usize> for WrappingArray<T> {
    type Output = T;

    fn index(&self, i: usize) -> &Self::Output {
        &self.0[i % self.0.len()]
    }
}

impl FromStr for WrappingArray<Tile> {
    type Err = char;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s.chars().map(Tile::try_from).collect::<Result<_, _>>()?;
        Ok(Self(v))
    }
}

impl<T> FromIterator<T> for WrappingArray<T> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Self(iter.into_iter().collect())
    }
}

#[derive(Clone, Copy)]
struct Algorithm {
    right: usize,
    down: usize,
}

type Map = WrappingArray<WrappingArray<Tile>>;

fn run_algorithm(map: &Map, alg: Algorithm) -> usize {
    let mut right = 0;
    let mut down = 0;
    let mut tree_count = 0;
    while down < map.0.len() {
        tree_count += map[down][right] as usize;
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
        .filter_map(|x| x.parse().ok())
        .collect();

    println!(
        "{}",
        ALGORITHMS
            .iter()
            .map(|alg| run_algorithm(&map, *alg))
            .product::<usize>()
    );
    Ok(())
}
