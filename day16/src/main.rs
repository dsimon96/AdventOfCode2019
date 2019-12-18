use std::io::{stdin, BufRead};
use std::iter::{repeat, Iterator};

fn gen_seq(i: usize) -> impl Iterator<Item = i64> {
    repeat(0)
        .take(i)
        .chain(
            repeat(1)
                .take(i)
                .chain(repeat(0).take(i).chain(repeat(-1).take(i))),
        )
        .cycle()
        .skip(1)
}

fn main() {
    let input_seq: Vec<i64> = stdin()
        .lock()
        .lines()
        .next()
        .expect("No input")
        .unwrap()
        .chars()
        .map(|c| i64::from(c.to_digit(10).expect("Invalid int")))
        .collect();

    // part 1
    let mut seq: Vec<i64> = input_seq.clone();

    for _ in 0..100 {
        seq = (1..=seq.len())
            .map(|i| -> i64 { seq.iter().zip(gen_seq(i)).map(|(x, y)| x * y).sum() })
            .map(|i| i.abs() % 10)
            .collect();
    }

    let first_eight: Vec<i64> = seq.into_iter().take(8).collect();
    println!("{:?}", first_eight);

    // part 2
    let repeated_len = input_seq.len() * 10000;
    seq = input_seq.into_iter().cycle().take(repeated_len).collect();

    let mut offset: usize = seq
        .iter()
        .take(7)
        .clone()
        .enumerate()
        .map(|(i, x)| 10usize.pow((6 - i) as u32) * (*x as usize))
        .sum();

    // This sucks, but it always works
    assert!(repeated_len % 2 == 0);
    assert!(offset >= repeated_len / 2);
    offset -= repeated_len / 2;

    seq.reverse();
    seq.truncate(repeated_len / 2);

    for _ in 0..100 {
        seq = seq
            .iter()
            .scan(0, |state, &x| {
                *state = (*state + x).abs() % 10;
                Some(*state)
            })
            .collect();
    }

    seq.reverse();
    let message: Vec<i64> = seq.into_iter().skip(offset).take(8).collect();
    println!("{:?}", message);
}
