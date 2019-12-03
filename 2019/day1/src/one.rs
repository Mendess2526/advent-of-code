use std::io::{stdin, BufRead, BufReader};

fn main() {
    println!(
        "total fuel: {}",
        BufReader::new(stdin())
            .lines()
            .flat_map(Result::ok)
            .map(|x| x.parse::<u32>().expect("Bad number"))
            .map(|d| d / 3 - 2)
            .sum::<u32>()
    );
}
