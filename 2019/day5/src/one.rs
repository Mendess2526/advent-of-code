use std::{
    convert::TryFrom,
    io::{self, stdin, stdout, Write},
};

struct Parammeter(i64, Mode);

impl Parammeter {
    fn target<'a, 'b: 'a>(&'a mut self, code: &'b mut [i64]) -> &'a mut i64 {
        match self.1 {
            Mode::Position => &mut code[usize::try_from(self.0).unwrap()],
            Mode::Immediate => &mut self.0,
        }
    }
}

#[derive(Debug)]
enum Mode {
    Position = 0,
    Immediate = 1,
}

impl Default for Mode {
    fn default() -> Self {
        Self::Position
    }
}

impl From<i64> for Mode {
    fn from(i: i64) -> Self {
        match i {
            0 => Self::Position,
            1 => Self::Immediate,
            _ => panic!("Invalid mode"),
        }
    }
}

enum Instruction {
    Add(Parammeter, Parammeter, Parammeter),
    Mul(Parammeter, Parammeter, Parammeter),
    Read(Parammeter),
    Write(Parammeter),
    Halt,
}

impl Instruction {
    fn decode(s: &[i64]) -> Self {
        match s[0] % 100 {
            99 => Self::Halt,
            1 => Self::Add(
                Parammeter(s[1], ((s[0] / 100) % 10).into()),
                Parammeter(s[2], ((s[0] / 1000) % 10).into()),
                Parammeter(s[3], ((s[0] / 10000) % 10).into()),
            ),
            2 => Self::Mul(
                Parammeter(s[1], ((s[0] / 100) % 10).into()),
                Parammeter(s[2], ((s[0] / 1000) % 10).into()),
                Parammeter(s[3], ((s[0] / 10000) % 10).into()),
            ),
            3 => Self::Read(Parammeter(s[1], ((s[0] / 100) % 10).into())),
            4 => Self::Write(Parammeter(s[1], ((s[0] / 100) % 10).into())),
            _ => panic!("Invalid opcode: {} from {}", s[0] % 100, s[0]),
        }
    }

    fn act(self, code: &mut [i64], it: usize) -> Option<usize> {
        match self {
            Self::Add(mut lhs, mut rhs, mut out) => {
                *out.target(code) = *lhs.target(code) + *rhs.target(code);
                Some(it + 4)
            }
            Self::Mul(mut lhs, mut rhs, mut out) => {
                *out.target(code) = *lhs.target(code) * *rhs.target(code);
                Some(it + 4)
            }
            Self::Read(mut out) => {
                let mut line = String::new();
                stdin().read_line(&mut line).expect("Error reading a line");
                *out.target(code) = line.trim().parse().expect("Error parsing input");
                Some(it + 2)
            }
            Self::Write(mut out) => {
                writeln!(stdout(), "{}", *out.target(code)).expect("Error writing to output");
                Some(it + 2)
            }
            Self::Halt => None,
        }
    }
}

fn main() -> io::Result<()> {
    let mut code = include_str!("../input")
        .split(',')
        .map(|d| d.trim().parse::<i64>())
        .map(Result::unwrap)
        .collect::<Vec<_>>();

    let mut it = 0;
    Ok(println!(
        "code[0] = {}",
        loop {
            match Instruction::decode(&mut code[it..]).act(&mut code, it) {
                Some(new_it) => it = new_it,
                None => break code[0],
            }
        }
    ))
}
