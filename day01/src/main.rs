use std::cmp::max;
use std::io::{self, BufRead};

fn fuel_for_module(mass: u64) -> u64 {
    let mut outstanding = mass;
    let mut fuel = max(outstanding / 3, 2) - 2;
    let mut total = 0;

    while fuel > 0 {
        total += fuel;
        outstanding = fuel;
        fuel = max(outstanding / 3, 2) - 2;
    }

    total
}

fn main() {
    let stdin = io::stdin();
    let res: u64 = stdin.lock()
        .lines()
        .map(|line| fuel_for_module(line.unwrap().parse::<u64>().expect("Invalid mass")))
        .sum();

    println!("Required fuel: {}", res);
}
