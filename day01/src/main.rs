use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashSet;

fn get_changes(f :File) -> Vec<i32> {
    BufReader::new(f)
        .lines()
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap().parse::<i32>().unwrap())
        .collect()
}

fn main() -> std::io::Result<()>{
    let mut freq :i32 = 0;
    let mut freqs = HashSet::new();
    for c in get_changes(File::open("input")?).iter().cycle() {
        freq += c;
        if freqs.contains(&freq) {
            break;
        }else{
            freqs.insert(freq);
        }
    }
    println!("Freq: {}", freq);
    Ok(())
}
