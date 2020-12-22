use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

struct Cenas {
    min: usize,
    max: usize,
    c: char,
    pass: String,
}

fn parse(s: String) -> Result<Cenas, Box<dyn Error>> {
    let cenas = s.split(' ').collect::<Vec<_>>();
    let bounds = cenas[0]
        .split_terminator("-")
        .map(|x| x.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Cenas {
        min: bounds[0],
        max: bounds[1],
        c: cenas[1].as_bytes()[0] as char,
        pass: cenas[2].to_owned(),
    })
}

fn valid(cenas: &Cenas) -> bool {
    let count = cenas.pass.chars().filter(|x| *x == cenas.c).count();
    count >= cenas.min && count <= cenas.max
}

fn valid2(cenas: &Cenas) -> bool {
    (cenas.pass.as_bytes()[cenas.min - 1] as char == cenas.c)
        != (cenas.pass.as_bytes()[cenas.max - 1] as char == cenas.c)
}

fn common<F>(f: F) -> io::Result<usize>
where
    F: Fn(&Cenas) -> bool,
{
    Ok(BufReader::new(File::open("input")?)
        .lines()
        .filter_map(Result::ok)
        .map(|x| parse(x))
        .filter_map(Result::ok)
        .filter(f)
        .count())
}

fn main() -> io::Result<()> {
    println!("{}", common(valid)?);
    println!("{}", common(valid2)?);
    Ok(())
}
