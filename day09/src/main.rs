use std::io::BufReader;
use std::io::Read;
use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut line = String::new();
    BufReader::new(File::open("input")?).read_to_string(&mut line)?;
    let words = line.split_whitespace().collect::<Vec<&str>>();
    let n_players = words[0].parse::<usize>().unwrap();
    let n_marbles = words[6].parse::<usize>().unwrap() * 100 + 1;
    let mut marbles = vec![0,1];
    let mut scores = vec![0;n_players];
    let mut marble_idx = 1;
    let mut current_plr = 0;
    for marble in 2..n_marbles {
        if marble % 23 == 0 {
            scores[current_plr] += marble;
            let bonus = if marble_idx < 7 {
                marbles.len() - (7 - marble_idx)
            } else {
                marble_idx - 7
            };
            scores[current_plr] += marbles.remove(bonus);
            marble_idx = bonus;
        }else{
            if marble_idx + 2 == marbles.len() {
                marbles.push(marble);
                marble_idx = marbles.len() - 1;
            } else {
                marble_idx = (marble_idx + 2) % marbles.len();
                marbles.insert(marble_idx, marble);
            }
        }
        current_plr = (current_plr + 1) % n_players;
        print!("{:08} : {:08.*}%", marble, 4, (marble as f64 / n_marbles as f64) * 100.0);
        print!("{}", std::iter::repeat('\x08').take(20).collect::<String>());
        std::io::stdout().flush().expect("couldn't flush");
    }
    let result :String = format!("{}", scores.iter().max().unwrap());
    println!("\n{}", result);
    File::create("output")?.write(result.as_bytes())?;
    Ok(())
}
