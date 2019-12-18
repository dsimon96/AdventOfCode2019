use std::cmp::min;
use std::env::args;

fn to_digit_seq(n: usize) -> Vec<usize> {
    let mut result: Vec<usize> = vec![0; 6];

    let mut remaining = n;
    for i in 0..6 {
        result[5 - i] = remaining % 10;
        remaining /= 10;
    }

    result
}

fn nondecreasing(s: &Vec<usize>) -> bool {
    s.windows(2).all(|pair| pair[0] <= pair[1])
}

fn has_adj_repeat(s: &Vec<usize>) -> bool {
    (0..10 as usize)
        .map(|d| s.iter().filter(|&n| *n == d).count())
        .any(|c| c == 2)
}

fn main() {
    let args: Vec<String> = args().collect();
    let lower = args[1].parse::<usize>().expect("invalid lower");
    let upper = min(args[2].parse::<usize>().expect("invalid upper") + 1, 999999);

    let result = (lower..upper + 1)
        .map(|n| to_digit_seq(n))
        .filter(|s| nondecreasing(s))
        .filter(|s| has_adj_repeat(s))
        .count();

    println!("{}", result);
}
