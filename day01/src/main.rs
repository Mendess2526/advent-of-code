use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashSet;

fn main() -> std::io::Result<()>{
    let mut freq :i32 = 0;
    BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|x| x.ok())
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
        .iter()
        .cycle()
        .try_fold(HashSet::new(), |mut freqs, c| {
            freq += c;
            if freqs.contains(&freq) {
                None
            }else{
                freqs.insert(freq);
                Some(freqs)
            }
        });
    println!("Freq: {}", freq);
    Ok(())
}
