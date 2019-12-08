#![feature(bool_to_option)]
use std::{
    convert::TryFrom,
    io::{self, BufRead, BufReader, Read, Write},
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

struct Pipe0(Receiver<i64>);
struct Pipe1(Sender<i64>);

impl Read for Pipe0 {
    fn read(&mut self, mut buf: &mut [u8]) -> io::Result<usize> {
        let v = self
            .0
            .recv()
            .map_err(|e| io::Error::new(io::ErrorKind::BrokenPipe, e))?;
        let len = buf.write(format!("{}\n", v).as_bytes())?;
        Ok(len)
    }
}

impl Write for Pipe1 {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if buf.len() == 1 && buf[0] == b'\n' { // This keeps being called with lonenly '\n'
            return Ok(1);
        }
        self.0
            .send(
                std::str::from_utf8(buf)
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?
                    .trim()
                    .parse::<i64>()
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?,
            )
            .map_err(|e| io::Error::new(io::ErrorKind::BrokenPipe, e))?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn pipe() -> (Pipe0, Pipe1) {
    let (s, r) = mpsc::channel();
    (Pipe0(r), Pipe1(s))
}

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

    fn act<In: Read, Out: Write>(
        self,
        code: &mut [i64],
        it: usize,
        stdin: &mut BufReader<In>,
        mut stdout: Out,
    ) -> Option<usize> {
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
                stdin.read_line(&mut line).expect("Error reading a line");
                *out.target(code) = line.trim().parse().expect("Error parsing input");
                Some(it + 2)
            }
            Self::Write(mut out) => {
                writeln!(stdout, "{}", *out.target(code)).expect("Error writing to output");
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

pub fn run<In: Read, Out: Write>(mut code: Vec<i64>, stdin: In, mut stdout: Out) {
    let mut it = 0;
    let mut stdin = BufReader::new(stdin);
    loop {
        match Instruction::decode(&mut code[it..]).act(&mut code, it, &mut stdin, &mut stdout) {
            Some(new_it) => it = new_it,
            None => break,
        }
    }
}

fn all_unique(s: &[i64; 5]) -> bool {
    for i in 0..s.len() {
        if s[i + 1..].iter().find(|x| **x == s[i]).is_some() {
            return false;
        }
    }
    return true;
}

fn main() -> io::Result<()> {
    let code = include_str!("../input")
        .split(',')
        .map(|d| d.trim().parse::<i64>())
        .map(Result::unwrap)
        .collect::<Vec<_>>();
    let mut phase_seq = [3, 3, 2, 1, 0];
    println!(
        "{}",
        std::iter::from_fn(|| {
            if phase_seq.iter().all(|i| *i == 4) {
                return None;
            }
            for i in &mut phase_seq {
                *i += 1;
                if *i == 5 {
                    *i = 0
                } else {
                    break;
                }
            }
            Some(phase_seq)
        })
        .filter(all_unique)
        .map(|seq| {
            let (output, mut old_p) = pipe();
            for i in seq.iter().rev() {
                let (pp0, mut pp1) = pipe();
                write!(pp1, "{}", i).expect("Error writing initial value");
                let code_clone = code.clone();
                thread::spawn(move || run(code_clone, pp0, old_p));
                old_p = pp1;
            }
            write!(old_p, "0").expect("Error writing initial input");
            output.0.recv().expect("Error reciving final output")
        })
        .max()
        .unwrap()
    );
    Ok(())
}
