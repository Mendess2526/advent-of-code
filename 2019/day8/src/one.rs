use std::mem::{size_of, transmute};

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

const INPUT: &[u8] = include_bytes!("../input");

const NLAYERS: usize = INPUT.len() / size_of::<Layer>();

#[repr(transparent)]
struct Layer([u8; WIDTH * HEIGHT]);

fn main() {
    println!(
        "{}",
        unsafe { transmute::<_, &[Layer; NLAYERS]>(INPUT.as_ptr()) }
            .iter()
            .min_by_key(|l| l.0.iter().filter(|c| **c == b'0').count())
            .map(|l| l.0.iter().fold((0, 0), |(ones, twos), x| {
                match x {
                    b'1' => (ones + 1, twos),
                    b'2' => (ones, twos + 1),
                    _ => (ones, twos),
                }
            }))
            .map(|(o, t)| o * t)
            .unwrap()
    )
}
