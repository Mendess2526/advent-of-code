use std::{
    fmt::{self, Display},
    mem::{size_of, transmute},
    ops::{Deref, DerefMut},
};

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

const INPUT: &[u8] = include_bytes!("../input");

const NLAYERS: usize = INPUT.len() / size_of::<Layer>();

#[repr(transparent)]
#[derive(Debug)]
struct Layer([[Kind; WIDTH]; HEIGHT]);

#[repr(u8)]
#[allow(dead_code)]
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum Kind {
    Black = b'1',
    White = b'0',
    Transparent = b'2',
}

impl Default for Layer {
    fn default() -> Self {
        Self([[Kind::Transparent; WIDTH]; HEIGHT])
    }
}

impl Deref for Layer {
    type Target = [[Kind; WIDTH]; HEIGHT];
    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

impl DerefMut for Layer {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.0
    }
}

impl Display for Layer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.iter() {
            for cell in line {
                match cell {
                    Kind::Black => write!(f, "X")?,
                    _ => write!(f, " ")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    println!(
        "{}",
        unsafe { transmute::<_, &[Layer; NLAYERS]>(INPUT.as_ptr()) }
            .iter()
            .fold(Layer::default(), |mut image, layer| {
                image
                    .iter_mut()
                    .flatten()
                    .zip(layer.iter().flatten())
                    .filter(|(i, _)| **i == Kind::Transparent)
                    .for_each(|(i, l)| *i = *l);
                image
            })
    )
}
