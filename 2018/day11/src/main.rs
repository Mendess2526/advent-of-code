fn calc_power(x :i32, y :i32, serial_number :i32) -> i32 {
    let mut v = 0;
    v += x + 10;
    v *= y;
    v += serial_number;
    v *= x + 10;
    v = (v / 100) - ((v / 1000) * 10);
    v - 5
}

fn calc_power_area(x :i32, y :i32, l :i32, c :i32, v :&Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    for i in x..l {
        for j in y..c {
            sum += v[i as usize][j as usize];
        }
    }
    sum
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        eprintln!("Usage: {} input", args[0]);
        return
    };
    let input = match args[1].parse::<i32>().ok() {
        None => panic!("Invalid input: {}", args[1]),
        Some(i) => i,
    };
    // for x in 0..11 {
    //     print!("[{:3}]", x);
    //     for y in 1..11 {
    //         if x == 0 {
    //             print!("[{:2}]", y);
    //         } else {
    //             print!(" {:3}", calc_power(x, y, input));
    //         }
    //     }
    //     println!();
    // }
    let mut biggest = i32::min_value();
    let mut big_x = 0;
    let mut big_y = 0;
    let mut size = 0;
    let mut matrix = vec![vec![]];
    for x in 1..301 {
        matrix.push(vec![]);
        for y in 1..301 {
            matrix[x].push(calc_power(x as i32, y, input));
        }
    }
    for x in 1..301 {
        for y in 1..301 {
            println!("{},{}", x, y);
            for s in 1..(301 - x.max(y)) {
                let area = calc_power_area(x, y, x + s, y + s, &matrix);
                if area > biggest {
                    biggest = area;
                    big_x = x;
                    big_y = y;
                    size = s;
                }
            }
        }
    }
    // let v = (y..(y+3)).flat_map(|y| std::iter::repeat(y).zip(x..(x+3)))
    //     .map(|(y,x)| calc_power(x, y, input)).sum();
    // if v > biggest { big_x = x; big_y = y; biggest = v; }
    println!("{}, {}, {}, {}", biggest, big_x, big_y, size);
}
