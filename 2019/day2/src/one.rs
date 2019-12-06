use std::io::{stdin, BufRead, BufReader};

fn main() {
    let mut code = BufReader::new(stdin().lock())
        .split(b',')
        .filter_map(Result::ok)
        .map(|d| {
            std::str::from_utf8(&d)
                .expect("invalid number")
                .trim()
                .parse::<usize>()
                .expect("invalid number")
        })
        .collect::<Vec<usize>>();
    let mut it = 0;
    let op: [fn(usize, usize) -> usize; 3] = [
        |_, _| unreachable!(),
        |lhs, rhs| lhs + rhs,
        |lhs, rhs| lhs * rhs,
    ];
    println!(
        "{}",
        loop {
            let o = code[it] as usize;
            match o {
                99 => break code[0],
                1 | 2 => {
                    let lhs = code[it + 1];
                    let rhs = code[it + 2];
                    let out = code[it + 3];
                    code[out] = op[o](code[lhs], code[rhs]);
                    it += 4;
                }
                _ => unreachable!("{}", o),
            }
        }
    )
}
