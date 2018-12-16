use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashMap;
use std::collections::BTreeSet;
use std::ops::BitXor;

fn parse_instruction(s :&[u8]) -> (char, char) {(s[5] as char, s[36] as char)}

fn main() -> std::io::Result<()> {
    let mut graph :HashMap<char, BTreeSet<char>> = HashMap::new();
    let mut jobs_with_req :BTreeSet<char> = BTreeSet::new();
    BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|d| d.map(|x| parse_instruction(x.as_bytes())).ok())
        .for_each(|(requirement, job)| {
            graph.entry(requirement).or_insert(BTreeSet::new()).insert(job);
            graph.entry(job).or_insert(BTreeSet::new());
            jobs_with_req.insert(job);
        });
    let mut sequence :Vec<char> = vec![];
    let mut todo = jobs_with_req.bitxor(&graph.keys().cloned().collect::<BTreeSet<_>>());
    while !todo.is_empty() {
        let job :char = todo.iter().cloned().next().unwrap();
        todo.remove(&job);
        let mut new_req = graph
            .remove(&job).unwrap_or(BTreeSet::new())
            .into_iter()
            .filter(|j| !graph.values().any(|m| m.contains(j)))
            .collect();
        todo.append(&mut new_req);
        sequence.push(job);
    }
    println!("{}", String::from_utf8(sequence.iter().map(|x| *x as u8).collect()).unwrap());
    Ok(())
}
