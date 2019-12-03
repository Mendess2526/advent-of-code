use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

#[derive(Debug)]
struct Tree {
    metadata: Vec<u32>,
    nodes: Vec<Tree>,
}

// file format: #nodes #meta {other nodes} meta
impl Tree {
    fn new(data :&[u32]) -> (Self, &[u32]) {
        let n_nodes = data[0];
        let n_meta = data[1];
        let mut nodes = vec![];
        let mut rest = &data[2..];
        for _ in 0..n_nodes {
            let (new_n, new_rest) = Tree::new(rest);
            nodes.push(new_n);
            rest = new_rest;
        }
        (Tree {
            metadata : Vec::from(&rest[0..(n_meta as usize)]),
            nodes: nodes,
        }, &rest[(n_meta as usize)..])
    }
    fn count(&self) -> u32 {
        self.nodes.iter().map(Tree::count).chain(self.metadata.iter().cloned()).sum()
    }
    fn special_count(&self) -> u32 {
        if self.nodes.is_empty() {
            self.metadata.iter().sum()
        } else {
            self.metadata
                .iter()
                .filter_map(|n| self.nodes.get(*n as usize - 1))
                .map(|n| n.special_count())
                .sum()
        }
    }
}

fn main() -> std::io::Result<()> {
    let words :Vec<u32> = BufReader::new(File::open("input")?)
        .split(b' ')
        .flat_map(|w| w.into_iter()
            .flat_map(|w| String::from_utf8(w))
            .flat_map(|w| w.parse::<u32>())
        )
        .collect();
    let (tree, _) = Tree::new(&words);
    println!("{}", tree.count());
    println!("{}", tree.special_count());
    Ok(())
}
