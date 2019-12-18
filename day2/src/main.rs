use std::io::{self, BufRead};

fn add(contents: &mut Vec<i64>, pc: &mut usize) {
    let lhs = contents[*pc+1] as usize;
    let rhs = contents[*pc+2] as usize;
    let out = contents[*pc+3] as usize;

    contents[out] = contents[lhs] + contents[rhs];

    *pc += 4;
}

fn mul(contents: &mut Vec<i64>, pc: &mut usize) {
    let lhs = contents[*pc+1] as usize;
    let rhs = contents[*pc+2] as usize;
    let out = contents[*pc+3] as usize;

    contents[out] = contents[lhs] * contents[rhs];

    *pc += 4;
}

fn main() {
    let contents: Vec<i64> = io::stdin()
        .lock()
        .lines()
        .nth(0)
        .expect("No input")
        .unwrap()
        .split(',')
        .map(|w| w.parse::<i64>().expect("invalid int"))
        .collect();

    for noun in 0..100 {
        for verb in 0..100 {
            let mut mem = contents.clone();
            mem[1] = noun;
            mem[2] = verb;
            let mut pc = 0;

            loop {
                let opcode = mem[pc];

                match opcode {
                    1 => add(&mut mem, &mut pc),
                    2 => mul(&mut mem, &mut pc),
                    99 => break,
                    _ => panic!("Invalid opcode!")
                };
            }

            if mem[0] == 19690720 {
                println!("100 * {} + {} = {}", noun, verb, 100 * noun + verb);
                return;
            }
        }
    }

    println!("No working pair!");
}
