use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() -> std::io::Result<()> {
    println!("{}", BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|x| x.ok())
        .map(|x| x.parse::<i32>().unwrap())
        .fold(0, |mut acc, c| {acc += c; acc})
        );
    Ok(())
}
