#![feature(bool_to_option)]
use std::{
    convert::TryFrom,
    io::{self, stdin, stdout, Write},
    ops::{Deref, DerefMut, Index, IndexMut},
};

type CellType = i128;

struct Code(Vec<CellType>);

impl Deref for Code {
    type Target = [CellType];
    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

impl DerefMut for Code {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.0
    }
}

// impl Index<usize> for Code {
//     type Output = CellType;
//     fn index(&self, i: usize) -> &<Self as Index<usize>>::Output {
//         &self.0[i]
//     }
// }

// impl IndexMut<usize> for Code {
//     fn index_mut(&mut self, i: usize) -> &mut <Self as Index<usize>>::Output {
//         if self.len() < i {
//             self.0.resize_with(i, Default::default);
//         }
//         &mut self.0[i]
//     }
// }

impl Code {
    fn get_or_extend(&mut self, index: usize) -> &mut CellType {
        if self.len() <= index {
            self.0.resize_with(index + 1, Default::default);
        }
        &mut self.0[index]
    }
}

struct Memory {
    code: Code,
    it: usize,
    relative_base: usize,
}

#[derive(Debug)]
struct Parammeter(i128, Mode);

impl Parammeter {
    fn target<'a, 'b: 'a>(&'a mut self, mem: &'b mut Memory) -> &'a mut i128 {
        match self.1 {
            Mode::Position => mem.code.get_or_extend(usize::try_from(self.0).unwrap()),
            Mode::Immediate => &mut self.0,
            Mode::Relative => mem.code.get_or_extend(
                usize::try_from(self.0 + CellType::try_from(mem.relative_base).unwrap())
                    .expect(&format!("He's too powerfull {}!", self.0)),
            ),
        }
    }
}

#[derive(Debug)]
enum Mode {
    Position = 0,
    Immediate = 1,
    Relative = 2,
}

impl Default for Mode {
    fn default() -> Self {
        Self::Position
    }
}

impl From<i128> for Mode {
    fn from(i: i128) -> Self {
        match i {
            0 => Self::Position,
            1 => Self::Immediate,
            2 => Self::Relative,
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
    AdjustRelative(Parammeter),
    Halt,
}

impl Instruction {
    fn decode(s: &[i128]) -> Self {
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
            9 => Self::AdjustRelative(param()),
            _ => panic!("Invalid opcode: {} from {}", s[0] % 100, s[0]),
        }
    }

    fn act(self, mem: &mut Memory) -> Option<usize> {
        match self {
            Self::Add(mut lhs, mut rhs, mut out) => {
                *out.target(mem) = *lhs.target(mem) + *rhs.target(mem);
                Some(mem.it + 4)
            }
            Self::Mul(mut lhs, mut rhs, mut out) => {
                *out.target(mem) = *lhs.target(mem) * *rhs.target(mem);
                Some(mem.it + 4)
            }
            Self::Read(mut out) => {
                let mut line = String::new();
                stdin().read_line(&mut line).expect("Error reading a line");
                *out.target(mem) = line.trim().parse().expect("Error parsing input");
                Some(mem.it + 2)
            }
            Self::Write(mut out) => {
                writeln!(stdout(), "{}", *out.target(mem)).expect("Error writing to output");
                Some(mem.it + 2)
            }
            Self::JumpTrue(mut test, mut target) => Some(
                (*test.target(mem) != 0)
                    .then(usize::try_from(*target.target(mem)).expect("Invalid jump address"))
                    .unwrap_or(mem.it + 3),
            ),
            Self::JumpFalse(mut test, mut target) => Some(
                (*test.target(mem) == 0)
                    .then(usize::try_from(*target.target(mem)).expect("Invalid jump address"))
                    .unwrap_or(mem.it + 3),
            ),
            Self::LessThan(mut lhs, mut rhs, mut out) => {
                *out.target(mem) = (*lhs.target(mem) < *rhs.target(mem)).then(1).unwrap_or(0);
                Some(mem.it + 4)
            }
            Self::Equals(mut lhs, mut rhs, mut out) => {
                *out.target(mem) = (*lhs.target(mem) == *rhs.target(mem)).then(1).unwrap_or(0);
                Some(mem.it + 4)
            }
            Self::AdjustRelative(mut adj) => {
                mem.relative_base =
                    usize::try_from(i128::try_from(mem.relative_base).unwrap() + *adj.target(mem))
                        .unwrap();
                Some(mem.it + 2)
            }
            Self::Halt => None,
        }
    }
}

fn main() -> io::Result<()> {
    let code = include_str!("../input")
        .split(',')
        .map(|d| d.trim().parse::<i128>())
        .map(Result::unwrap)
        .collect::<Vec<_>>();

    let mut mem = Memory {
        code: Code(code),
        it: 0,
        relative_base: 0,
    };
    Ok(println!(
        "code[0] = {}",
        loop {
            match Instruction::decode(&mem.code.0[mem.it..]).act(&mut mem) {
                Some(new_it) => mem.it = new_it,
                None => break mem.code[0],
            }
        }
    ))
}
