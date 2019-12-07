const INPUT: (usize, usize) = (246515, 739105);

fn break_num(i: usize) -> [u8; 6] {
    [
        ((i / 100000) % 10) as u8,
        ((i / 10000) % 10) as u8,
        ((i / 1000) % 10) as u8,
        ((i / 100) % 10) as u8,
        ((i / 10) % 10) as u8,
        ((i / 1) % 10) as u8,
    ]
}

trait Also
where
    Self: Sized,
{
    fn also<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut Self),
    {
        f(&mut self);
        self
    }
}

impl<T> Also for T {}

fn main() {
    println!(
        "{}",
        (INPUT.0..INPUT.1)
            .map(break_num)
            .filter(|i| {
                i.iter()
                    .skip(1)
                    .scan(0, |last, x| {
                        Some(*x == i[*last]).also(|_| *last = *last + 1)
                    })
                    .any(|b| b)
            })
            .filter(|i| {
                i.iter()
                    .skip(1)
                    .scan(0, |last, x| Some(*x >= i[*last]).also(|_| *last = *last + 1))
                    .all(|b| b)
            })
            .count()
    );
}
