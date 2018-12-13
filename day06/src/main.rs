use std::io::BufReader;
use std::fs::File;
use std::io::BufRead;
use std::collections::{HashMap, BTreeMap};

static mut ID :u8 = b'A';
/*
 *     y
 *     ^
 * 1st | 2nd
 * ---------> x
 * 3rd | 4th
 *     |
 */

#[derive(Debug, Hash, PartialEq, Eq)]
struct Dot {
    id: char,
    x: i16,
    y: i16,
    extreme: bool,
}

impl From<(i16,i16)> for Dot {
    fn from((x,y) :(i16,i16)) -> Self {
        let id;
        unsafe {
            id = ID as char;
            ID += 1;
        }
        Dot {
            id: id,
            x: x,
            y: y,
            extreme: false,
        }
    }
}

impl Dot {
    fn zero((x,y) :(i16,i16)) -> Self {
        Dot{
            id: 0 as char,
            x,
            y,
            extreme: false,
        }
    }
    fn fst_quad(&self, target :&(i16,i16)) -> bool {
        self.x > target.0 && self.y < target.1
    }

    fn snd_quad(&self, target :&(i16,i16)) -> bool {
        self.x < target.0 && self.y < target.1
    }

    fn thr_quad(&self, target :&(i16,i16)) -> bool {
        self.x > target.0 && self.y > target.1
    }

    fn for_quad(&self, target :&(i16,i16)) -> bool {
        self.x < target.0 && self.y > target.1
    }

    fn set_extreme(&mut self, selfs :&Vec<(i16,i16)>) {
        self.extreme = !selfs
            .iter()
            .any(|c| self.fst_quad(c) || self.snd_quad(c) || self.thr_quad(c) || self.for_quad(c));
    }
}

fn get_coords(s :String) -> (i16, i16) {
    let mut it = s.split(',');
    (
        it.next().map(|x| x.trim().parse().unwrap()).unwrap(),
        it.next().map(|x| x.trim().parse().unwrap()).unwrap()
    )
}

fn closest(p :(i16,i16), dots :&[Dot]) -> Option<&Dot> {
    let mut sorted_dots :Vec<(&Dot, i16)> =
        dots.iter().map(|d| (d, manhattan_dist(p, d))).collect();
    sorted_dots.sort_unstable_by_key(|d| d.1);
    sorted_dots.reverse();
    let (closest, c_dist) = sorted_dots.pop().unwrap();
    let (c, cd) = sorted_dots.pop().unwrap();
    if c_dist == cd { None } else { Some(closest) }
}

fn manhattan_dist((a, b) :(i16,i16), d2 :&Dot) -> i16 {
    let (x,y) = (d2.x, d2.y);
    (a-x).abs() + (b-y).abs()
}

fn main() -> std::io::Result<()> {
    let mut dots :Vec<Dot> = BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|x| x.ok().map(|l| get_coords(l)))
        .map(|c| Dot::from(c))
        .collect();
    for d in dots.iter() {
        println!("{:?}", d);
    }
    let coords = dots.iter().map(|d| (d.x,d.y)).collect();
    dots.iter_mut().for_each(|d| d.set_extreme(&coords));
    let max_x = dots.iter().max_by_key(|d| d.x).map(|d| d.x).unwrap() as usize;
    let max_y = dots.iter().max_by_key(|d| d.y).map(|d| d.y).unwrap() as usize;
    let min_x = dots.iter().min_by_key(|d| d.x).map(|d| d.x).unwrap() as usize;
    let min_y = dots.iter().min_by_key(|d| d.y).map(|d| d.y).unwrap() as usize;
    let mut matrix = vec![];
    for i in 0..(max_x - min_x) {
        matrix.push(vec![]);
        for j in 0..(max_y - min_y) {
            matrix[i].push(closest(((i + min_x) as i16, (j + min_y) as i16), &dots[..]));
        }
    }
    for line in matrix.iter() {
        for d in line.iter() {
            match d {
                None => print!("."),
                Some(v) => print!("{}", v.id),
            };
        }
        print!("\n");
    }
    let counts = matrix.iter()
        .flat_map(|l| l.iter())
        .filter_map(|d| *d)
        .fold(HashMap::new(), |mut acc, d| {
            acc.entry(d).and_modify(|c| *c += 1).or_insert(1);
            acc
        });
    let mut sorted_dots = BTreeMap::new();
    counts.iter().for_each(|(k, v)| { sorted_dots.insert(v,k); () });
    for kv in sorted_dots.iter().rev() {
        if !kv.1.extreme {
            println!("{:?}", kv);
            break;
        };
    }
    Ok(())
}
