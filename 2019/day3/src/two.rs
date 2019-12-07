use std::{
    collections::HashSet,
    io::{stdin, BufRead, BufReader},
    iter::successors,
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
    fn next(self, trail: Trail) -> Vec<Trail> {
        match self.kind {
            Kind::D => successors(Some(trail.inc_x()), |last| Some(last.inc_x()))
                .take(self.times)
                .collect(),
            Kind::R => successors(Some(trail.inc_y()), |last| Some(last.inc_y()))
                .take(self.times)
                .collect(),
            Kind::U => successors(Some(trail.dec_x()), |last| Some(last.dec_x()))
                .take(self.times)
                .collect(),
            Kind::L => successors(Some(trail.dec_y()), |last| Some(last.dec_y()))
                .take(self.times)
                .collect(),
        }
    }
}

#[derive(Clone, Copy, Default, Debug)]
struct Trail {
    pos: (i64, i64),
    steps: usize,
}

impl Trail {
    fn inc_x(self) -> Self {
        Self {
            pos: (self.pos.0 + 1, self.pos.1),
            steps: self.steps + 1,
        }
    }

    fn dec_x(self) -> Self {
        Self {
            pos: (self.pos.0 - 1, self.pos.1),
            steps: self.steps + 1,
        }
    }

    fn inc_y(self) -> Self {
        Self {
            pos: (self.pos.0, self.pos.1 + 1),
            steps: self.steps + 1,
        }
    }

    fn dec_y(self) -> Self {
        Self {
            pos: (self.pos.0, self.pos.1 - 1),
            steps: self.steps + 1,
        }
    }
}

impl std::hash::Hash for Trail {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pos.hash(state)
    }
}

impl PartialEq for Trail {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

impl std::ops::Add for Trail {
    type Output = Self;
    fn add(self, other: Self) -> <Self as std::ops::Add>::Output {
        Self {
            steps: self.steps + other.steps,
            ..self
        }
    }
}

impl Eq for Trail {}

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
                    .fold((HashSet::new(), Trail::default()), |(mut set, last), x| {
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
                    acc.iter()
                        .filter_map(|a| x.get(a).map(|xa| *xa + *a))
                        .collect()
                }
            })
            .iter()
            .min_by_key(|t| t.steps)
    )
}
