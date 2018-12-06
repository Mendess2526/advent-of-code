use std::collections::HashMap;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn is_start_shift(line :&str) -> bool {
    line.as_bytes()[19] == b'G'
}

fn parse_guard_id(line :&str) -> u32 {
    line[26..].split(' ').next().unwrap().parse().unwrap()
}

fn parse_event(line :&str) -> (u32, bool) {
    let minute = if &line[12..14] != "00" {
        0
    }else{
        line[15..17].parse().unwrap()
    };
    (minute, line.as_bytes()[19] == b'f')
}

fn add_minutes(guard :&mut [u32;60], from :u32, to :u32) -> u32 {
    for i in from..to {
        guard[i as usize] += 1;
    }
    to
}

fn max_index(v :Vec<u32>) -> u32 {
    let mut max = 0;
    let mut max_i :u32 = 0;
    for i in 0..v.len() {
        if v[i] > max {
            max = v[i];
            max_i = i as u32;
        }
    };
    max_i
}

fn main() -> std::io::Result<()> {
    let lines = BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|x| x.ok())
        .collect::<BTreeSet<String>>();

    let mut guards :HashMap<u32,[u32;60]> = HashMap::new();
    let mut start_shift;
    let mut guard_id = 0;
    let mut min = 0;
    for line in lines.iter() {
        start_shift = is_start_shift(&line);
        if start_shift {
            guard_id = parse_guard_id(&line);
            min = 0;
        } else {
            match parse_event(&line) {
                (m, true) => min = m,
                (m, false) => {
                    let guard = guards.entry(guard_id).or_insert([0;60]);
                    min = add_minutes(guard, min, m)
                },
            };
        }
    };
    {
        let (id, _) = guards.iter()
            .map(|(id, v)| (id, v.iter().sum::<u32>()))
            .max_by(|(_,v1), (_, v2)| v1.cmp(&v2))
            .unwrap();
        let max_i = max_index(guards.get(id).unwrap().to_vec());
        println!("{}", (*id * max_i) as u32);
    }
    {
        let (id, _) = guards.iter()
            .map(|(id, v)| (id, v.iter().max()))
            .max_by(|(_, v1), (_, v2)| v1.cmp(&v2))
            .unwrap();
        let max_i = max_index(guards.get(id).unwrap().to_vec());
        println!("{}", (*id * max_i) as u32);
    }
    Ok(())
}
