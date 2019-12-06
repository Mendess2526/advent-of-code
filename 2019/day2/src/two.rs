use std::io::{stdin, BufRead, BufReader};

fn calculate(mut code: Vec<usize>) -> usize {
    let mut it = 0;
    let op: [fn(usize, usize) -> usize; 3] = [
        |_, _| unreachable!(),
        |lhs, rhs| lhs + rhs,
        |lhs, rhs| lhs * rhs,
    ];
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
}

fn main() {
    let code = BufReader::new(stdin().lock())
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
    for noun in 0..99 {
        for verb in 0..99 {
            let mut n = code.clone();
            n[1..=2].copy_from_slice(&[noun, verb]);
            if calculate(n) == 19690720 {
                println!("100 * noun + verb: {}", 100 * noun + verb);
                break;
            }
        }
    }
}
