use std::io::BufReader;
use std::fs::File;
use std::io::BufRead;

fn reacting(l :&mut Vec<u8>) -> bool {
    let mut rm = -1;
    for i in 0..(l.len() - 1) {
        if l[i].max(l[i+1]) - l[i].min(l[i+1]) == 32 {
            rm = i as i64;
            break
        }
    }
    if rm == -1 { return false };
    l.remove(rm as usize);
    l.remove(rm as usize);
    true
}

fn main() -> std::io::Result<()> {
    {// part 1
        let mut line = String::new();
        BufReader::new(File::open("input")?)
            .read_line(&mut line)?;
        let mut l = String::from(line.trim()).into_bytes();
        while reacting(&mut l) {}
        println!("{}", l.len());

    }
    {// part 2
        let mut line = String::new();
        BufReader::new(File::open("input")?)
            .read_line(&mut line)?;
        let l = line.trim().as_bytes().to_vec();
        let mut lens = vec![];
        for c in b'A'..b'Z' {
            let mut f = l.iter().filter(|x| **x != c && **x-32 != c).cloned().collect();
            while reacting(&mut f) {}
            lens.push(f.len());
        }
        println!("{}", lens.iter().min().unwrap());
    }
    Ok(())
}
