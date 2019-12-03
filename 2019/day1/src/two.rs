use std::io::{stdin, BufRead, BufReader};

fn fuel_mass(base_fuel: u32) -> u32 {
    match base_fuel {
        0 => 0,
        n => n + fuel_mass((n / 3).saturating_sub(2))
    }
}

fn main() {
    println!(
        "total fuel: {}",
        BufReader::new(stdin())
            .lines()
            .flat_map(Result::ok)
            .map(|x| x.parse::<u32>().expect("Bad number"))
            .map(|d| (d / 3).saturating_sub(2))
            .map(fuel_mass)
            .sum::<u32>()
    );
}
