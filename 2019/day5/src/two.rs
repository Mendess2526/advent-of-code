#![feature(bool_to_option)]
use std::{
    convert::TryFrom,
    io::{self, stdin, stdout, Write},
};

#[derive(Debug)]
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

#[derive(Debug)]
enum Instruction {
    Add(Parammeter, Parammeter, Parammeter),
    Mul(Parammeter, Parammeter, Parammeter),
    Read(Parammeter),
    Write(Parammeter),
    JumpTrue(Parammeter, Parammeter),
    JumpFalse(Parammeter, Parammeter),
    LessThan(Parammeter, Parammeter, Parammeter),
    Equals(Parammeter, Parammeter, Parammeter),
    Halt,
}

impl Instruction {
    fn decode(s: &[i64]) -> Self {
        let mut m = 10;
        let mut mode = || {
            m *= 10;
            ((s[0] / m) % 10).into()
        };
        let mut i = 0;
        let mut p = || {
            i += 1;
            s[i]
        };
        let mut param = || Parammeter(p(), mode());
        match s[0] % 100 {
            99 => Self::Halt,
            1 => Self::Add(param(), param(), param()),
            2 => Self::Mul(param(), param(), param()),
            3 => Self::Read(param()),
            4 => Self::Write(param()),
            5 => Self::JumpTrue(param(), param()),
            6 => Self::JumpFalse(param(), param()),
            7 => Self::LessThan(param(), param(), param()),
            8 => Self::Equals(param(), param(), param()),
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
            Self::JumpTrue(mut test, mut target) => Some(
                (*test.target(code) != 0)
                    .then(usize::try_from(*target.target(code)).expect("Invalid jump address"))
                    .unwrap_or(it + 3),
            ),
            Self::JumpFalse(mut test, mut target) => Some(
                (*test.target(code) == 0)
                    .then(usize::try_from(*target.target(code)).expect("Invalid jump address"))
                    .unwrap_or(it + 3),
            ),
            Self::LessThan(mut lhs, mut rhs, mut out) => {
                *out.target(code) = (*lhs.target(code) < *rhs.target(code)).then(1).unwrap_or(0);
                Some(it + 4)
            }
            Self::Equals(mut lhs, mut rhs, mut out) => {
                *out.target(code) = (*lhs.target(code) == *rhs.target(code))
                    .then(1)
                    .unwrap_or(0);
                Some(it + 4)
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
