use std::process::Command;
use std::iter::repeat;

#[derive(Eq,PartialEq,PartialOrd,Ord,Debug)]
struct Point {
    y: i32,
    x: i32,
    vx: i32,
    vy: i32,
}

impl Point {
    fn commute(&mut self) { // move is a keyword :P
        self.x += self.vx;
        self.y += self.vy;
    }

    fn distance(&self, other :&Point) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as usize
    }

    fn overlap(&self, other :&Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl From<Vec<i32>> for Point {
    fn from(v :Vec<i32>) -> Self {
        Point{
            x: v[0],
            y: v[1],
            vx: v[2],
            vy: v[3],
        }
    }
}

fn draw(points :&Vec<Point>) -> bool {
    let smallest_x = points.iter().min_by_key(|p| p.x).map(|p| p.x).unwrap();
    let smallest_y = points.iter().min_by_key(|p| p.y).map(|p| p.y).unwrap();
    let biggest_x = points.iter().max_by_key(|p| p.x).map(|p| p.x).unwrap();
    let biggest_y = points.iter().max_by_key(|p| p.y).map(|p| p.y).unwrap();
    if (smallest_x - biggest_x).abs() > 159 || (biggest_y - smallest_y).abs() > 44 {
        return false;
    };
    let mut last = &Point{ y: i32::max_value(), x: 0, vx: 0, vy: 0 };
    println!("===");
    for p in points.iter() {
        if last.y != p.y {
            println!();
            print!("{}", repeat(' ').take((smallest_x - p.x).abs() as usize).collect::<String>());
            print!("#");
        } else if !last.overlap(&p) {
            print!("{}", repeat(' ').take(last.distance(&p) - 1).collect::<String>());
            print!("#");
        }
        last = &p;
    }
    println!();
    true
}

fn main() {
    let output = Command::new("sed")
        .args(&["s/position=<//g; s/> velocity=</ /g; s/>//g; s/,//g", "input"])
        .output()
        .expect("Couldn't spawn sed");
    let mut points :Vec<Point>= String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|l| l.split_whitespace()
             .filter_map(|w| w.parse::<i32>().ok())
             .collect::<Vec<i32>>())
        .map(|v| Point::from(v))
        .collect();
    let mut seconds = 0;
    loop{
        points.sort_unstable();
        if draw(&points) {
            let mut line = String::new();
            if let Err(e) = std::io::stdin().read_line(&mut line) {
                eprintln!("{}", e);
            }
            if line.contains("quit") { break };
        }
        for p in points.iter_mut() {
            p.commute();
        }
        seconds += 1;
    }
    println!("Total time: {}", seconds);
}
