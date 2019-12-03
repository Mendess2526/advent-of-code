use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashMap;
use std::collections::BTreeSet;
use std::boxed::Box;

fn main() -> std::io::Result<()> {
    {// star 1
        let mut graph = make_graph(File::open("input")?);
        let mut sequence :Vec<char> = vec![];
        let mut todo = get_available_tasks(&graph);
        while !todo.is_empty() {
            let job :char = todo.iter().cloned().next().unwrap();
            get_new_tasks(&mut todo, &mut graph, &[job]);
            sequence.push(job);
        }
        println!("{}", String::from_utf8(sequence.iter().map(|x| *x as u8).collect()).unwrap());
    }
    {// star 2
        const WORKERS :usize = 5;
        let mut graph = make_graph(File::open("input")?);
        let mut sequence :Vec<char> = vec![];
        let mut todo = get_available_tasks(&graph);
        let mut workers :Vec<Option<(char, i32)>> = vec![];
        for _ in 0..WORKERS { workers.push(None); }
        let mut time = -1;
        while !todo.is_empty() {
            let done = work(&mut workers);
            done.iter().cloned().for_each(|c| sequence.push(c));
            get_new_tasks(&mut todo, &mut graph, &done);
            let mut todo_iter = todo.iter();
            while idle(&workers) {
                match todo_iter.next() {
                    None => break,
                    Some(c) => assing_work(&mut workers, *c),
                };
            }
            println!("{:?}, {:?}", workers.iter().filter_map(|d| *d).collect::<Vec<_>>(), todo);
            time += 1;
        }
        println!("{}", time);
    }
    Ok(())
}

fn parse_instruction(s :&[u8]) -> (char, char) {(s[5] as char, s[36] as char)}

fn make_graph(file :File) -> Box<HashMap<char, BTreeSet<char>>> {
    let mut graph :HashMap<char, BTreeSet<char>> = HashMap::new();
    BufReader::new(file)
        .lines()
        .filter_map(|d| d.map(|x| parse_instruction(x.as_bytes())).ok())
        .for_each(|(requirement, job)| {
            graph.entry(requirement).or_insert(BTreeSet::new()).insert(job);
            graph.entry(job).or_insert(BTreeSet::new());
        });
    Box::new(graph)
}

fn get_available_tasks(graph :&HashMap<char, BTreeSet<char>>) -> Box<BTreeSet<char>> {
    Box::new(graph
             .keys()
             .cloned()
             .filter(|j| graph.values().all(|r| !r.contains(j)))
             .collect())
}

fn idle(workers :&Vec<Option<(char, i32)>>) -> bool {
    workers.iter().any(Option::is_none)
}

fn assing_work(workers :&mut Vec<Option<(char, i32)>>, task :char) {
    if workers.iter().filter_map(|d| *d).any(|(c, _)| c == task) { return; }
    for i in 0..workers.len() {
        if let None = workers[i] {
            workers[i] = Some((task, task_cost(task)));
            break;
        }
    }
}

fn task_cost(task :char) -> i32 { 61 + (task as i32 - 'A' as i32) }

fn work(workers :&mut Vec<Option<(char, i32)>>) -> Vec<char> {
    let mut done = vec![];
    for w in workers.iter_mut() {
        match *w {
            Some((c, 0)) => (),
            Some((c, i)) => {*w = Some((c, i - 1));},
            None => (),
        }
    }
    for w in workers.iter_mut() {
        match *w {
            Some((c, 0)) => {done.push(c); *w = None;},
            Some(_) => (),
            None => (),
        }
    }
    done
}

fn get_new_tasks(
    todo :&mut BTreeSet<char>,
    graph :&mut HashMap<char, BTreeSet<char>>,
    done :&[char])
{
    for job in done.iter() {
        todo.remove(&job);
        let mut new_req = graph
            .remove(&job).unwrap_or(BTreeSet::new())
            .into_iter()
            .filter(|j| graph.values().all(|m| !m.contains(j)))
            .collect();
        todo.append(&mut new_req);
    }
}
