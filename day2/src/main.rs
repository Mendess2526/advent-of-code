use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;

fn add_pairs((a,b) :(i32,i32), (x,y) :(i32,i32)) -> (i32,i32) { (a+x, b+y) }

fn count(s :&str) -> (i32,i32) {
    let mut m = HashMap::new();
    for c in s.chars() {
        m.entry(c)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    };
    let x = i32::from(m.values().any(|x| *x == 2));
    let y = i32::from(m.values().any(|x| *x == 3));
    (x,y)
}

fn get_id(buff :BufReader<File>) -> String {
    let mut ids :Vec<String> = vec![];
    for line in buff.lines() {
        match line {
            Ok(l) => {
                match ids.iter().find(|x| good_dist(&l, x)) {
                    Some(e) => return get_common(e, &l),
                    None => (),
                };
                ids.push(l);
            },
            Err(_) => (),
        };
    };
    "".to_string()
}

fn good_dist(a :&str, b :&str) -> bool {
    let mut count = 0;
    if a.len() != b.len() { return false };
    for (i,j) in a.chars().zip(b.chars()) {
        if i != j { count += 1; }
        if count == 2 { return false };
    }
    true
}

fn get_common(a :&str, b :&str) -> String {
    let mut r = String::new();
    for (i,j) in a.chars().zip(b.chars()) {
        if  i == j {
            r.push(i);
        }
    }
    return r;
}

fn main() -> std::io::Result<()> {
    let (x,y) = BufReader::new(File::open("input")?)
        .lines()
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        .fold((0,0), |acc, x| add_pairs(acc, count(&x)));
    println!("{}", x * y);

    println!("common: {}", get_id(BufReader::new(File::open("input")?)));
    Ok(())
}
