use std::{
    collections::HashSet,
    io::{stdin, BufRead, BufReader},
    str::FromStr,
};

#[derive(Copy, Clone, Debug)]
enum Kind {
    U,
    R,
    D,
    L,
}

impl FromStr for Kind {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match s {
            "U" => Ok(Self::U),
            "R" => Ok(Self::R),
            "D" => Ok(Self::D),
            "L" => Ok(Self::L),
            _ => Err(s.to_owned()),
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Instruction {
    kind: Kind,
    times: usize,
}

impl FromStr for Instruction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        Ok(Self {
            kind: s[0..1].parse::<Kind>()?,
            times: s[1..].parse::<usize>().map_err(|e| e.to_string())?,
        })
    }
}

impl Instruction {
    fn next(self, (x, y): (i64, i64)) -> Vec<(i64, i64)> {
        match self.kind {
            Kind::D => (1..=self.times as i64).map(|t| (x - t, y)).collect(),
            Kind::R => (1..=self.times as i64).map(|t| (x, y + t)).collect(),
            Kind::U => (1..=self.times as i64).map(|t| (x + t, y)).collect(),
            Kind::L => (1..=self.times as i64).map(|t| (x, y - t)).collect(),
        }
    }
}

fn main() {
    println!(
        "{:?}",
        BufReader::new(stdin())
            .lines()
            .filter_map(Result::ok)
            .map(|l| {
                l.split(',')
                    .map(Instruction::from_str)
                    .map(Result::unwrap)
                    .fold((HashSet::new(), (0, 0)), |(mut set, last), x| {
                        let next = x.next(last);
                        let last = *next.last().unwrap();
                        set.extend(next);
                        (set, last)
                    })
                    .0
            })
            .fold(HashSet::new(), |acc, x| {
                if acc.is_empty() {
                    x
                } else {
                    acc.intersection(&x).copied().collect()
                }
            })
            .iter()
            .map(|(x, y)| x.abs() + y.abs())
            .min()
    )
}
