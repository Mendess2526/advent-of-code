const INPUT: (usize, usize) = (246515, 739105);

fn main() {
    println!(
        "{}",
        (INPUT.0..INPUT.1)
            .map(|i| [
                ((i / 100000) % 10) as u8,
                ((i / 10000) % 10) as u8,
                ((i / 1000) % 10) as u8,
                ((i / 100) % 10) as u8,
                ((i / 10) % 10) as u8,
                ((i / 1) % 10) as u8,
            ])
            .filter(|i| {
                i.iter()
                    .skip(1)
                    .scan(0, |last, &x| {
                        let r = Some(x == i[*last])
                            .map(|rep| rep && (*last == 0 || i[*last] != i[*last - 1]))
                            .map(|rep| rep && (*last >= 4 || x != i[*last + 2]));
                        *last += 1;
                        r
                    })
                    .any(|b| b)
            })
            .filter(|i| {
                i.iter()
                    .skip(1)
                    .scan(0, |last, x| {
                        let r = Some(*x >= i[*last]);
                        *last += 1;
                        r
                    })
                    .all(|b| b)
            })
            .count()
    );
}
