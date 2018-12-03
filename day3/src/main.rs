use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;

type Matrix = Vec<Vec<Vec<u32>>>;

fn new_matrix(x :usize, y :usize) -> Matrix {
    let mut m = Vec::with_capacity(x);
    for i in 0..x {
        m.push(Vec::with_capacity(y));
        for _j in 0..y {
            m[i].push(Vec::new());
        }
    }
    m
}

fn parse_until(it :&mut Iterator<Item=char>, token :char) -> u32 {
    let mut parsed_str = String::new();
    for c in it {
        if c == token { break };
        parsed_str.push(c);
    }
    parsed_str.trim().parse::<u32>().unwrap()
}
fn parse_line(l :&str) -> (u32, u32, u32, u32 ,u32) {
    let mut it = l.chars();
    it.next(); // skip #
    (
        parse_until(&mut it, '@'),
        parse_until(&mut it, ','),
        parse_until(&mut it, ':'),
        parse_until(&mut it, 'x'),
        parse_until(&mut it, 'z')
    )
}

fn add_sheet(m :&mut Matrix, id :u32, x :u32, y :u32, sizex :u32, sizey :u32) {
    for i in x..(x+sizex) {
        for j in y..(y+sizey) {
            m[i as usize][j as usize].push(id);
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut m = new_matrix(1000,1000);
    BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|x| x.ok())
        .for_each(|l| {
            let (id, x, y, sizex, sizey) = parse_line(&l);
            add_sheet(&mut m, id, x, y, sizex, sizey);
        });
    let count :usize = m.iter()
        .map(|x| x.iter().filter(|x| x.len() > 1).count())
        .sum();
    println!("overlap: {}", count);
    let mut unoverlapped = HashMap::new();
    m.iter()
        .for_each(|l| l.iter().for_each(|c|
         match c.len() {
            0 => (),
            1 => if !unoverlapped.contains_key(&c[0]) { unoverlapped.insert(&c[0], true); },
            _ => c.iter().for_each(|id| { unoverlapped.insert(id, false); () }),
        }));
    unoverlapped.retain(|_, v| *v);
    println!("{:?}", unoverlapped);
    Ok(())
}
