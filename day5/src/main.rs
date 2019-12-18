use std::io::{self, BufRead};

fn get_arg(contents: &Vec<i64>, pc: usize, offset: usize) -> i64 {
    let opcode = contents[pc] as usize;
    let mode = opcode / 10usize.pow(1 + offset as u32) % 10;

    match mode {
        0 => {
            let pos = contents[pc + offset] as usize;
            contents[pos]
        }
        1 => contents[pc + offset],
        _ => panic!("Invalid mode"),
    }
}

fn get_args(contents: &Vec<i64>, pc: usize) -> (i64, i64, i64) {
    (
        get_arg(contents, pc, 1),
        get_arg(contents, pc, 2),
        contents[pc + 3],
    )
}

fn add(contents: &mut Vec<i64>, pc: &mut usize) -> bool {
    let (lhs, rhs, out) = get_args(&contents, *pc);

    contents[out as usize] = lhs + rhs;
    *pc += 4;

    false
}

fn mul(contents: &mut Vec<i64>, pc: &mut usize) -> bool {
    let (lhs, rhs, out) = get_args(&contents, *pc);

    contents[out as usize] = lhs * rhs;
    *pc += 4;

    false
}

fn get_input() -> i64 {
    io::stdin()
        .lock()
        .lines()
        .next()
        .expect("No input")
        .unwrap()
        .parse::<i64>()
        .expect("invalid int")
}

fn input(contents: &mut Vec<i64>, pc: &mut usize) -> bool {
    let out = contents[*pc + 1];

    contents[out as usize] = get_input();
    *pc += 2;

    false
}

fn output(contents: &mut Vec<i64>, pc: &mut usize) -> bool {
    let val = get_arg(contents, *pc, 1);

    println!("{}", val);
    *pc += 2;

    false
}

fn jump_if_true(contents: &mut Vec<i64>, pc: &mut usize) -> bool {
    let (lhs, rhs) = (get_arg(contents, *pc, 1), get_arg(contents, *pc, 2));

    if lhs != 0 {
        *pc = rhs as usize;
    } else {
        *pc += 3;
    }

    false
}

fn jump_if_false(contents: &mut Vec<i64>, pc: &mut usize) -> bool {
    let (lhs, rhs) = (get_arg(contents, *pc, 1), get_arg(contents, *pc, 2));

    if lhs == 0 {
        *pc = rhs as usize;
    } else {
        *pc += 3;
    }

    false
}

fn less_than(contents: &mut Vec<i64>, pc: &mut usize) -> bool {
    let (lhs, rhs, out) = get_args(contents, *pc);

    if lhs < rhs {
        contents[out as usize] = 1;
    } else {
        contents[out as usize] = 0;
    }
    *pc += 4;

    false
}

fn equals(contents: &mut Vec<i64>, pc: &mut usize) -> bool {
    let (lhs, rhs, out) = get_args(contents, *pc);

    if lhs == rhs {
        contents[out as usize] = 1;
    } else {
        contents[out as usize] = 0;
    }
    *pc += 4;

    false
}

fn ret(_contents: &mut Vec<i64>, _pc: &mut usize) -> bool {
    true
}

fn decode(contents: &Vec<i64>, pc: usize) -> fn(&mut Vec<i64>, &mut usize) -> bool {
    let opcode = contents[pc] as usize;
    let operation = opcode % 100;

    match operation {
        1 => add,
        2 => mul,
        3 => input,
        4 => output,
        5 => jump_if_true,
        6 => jump_if_false,
        7 => less_than,
        8 => equals,
        99 => ret,
        _ => panic!("Unrecognized opcode: {}", operation),
    }
}

fn main() {
    let contents: Vec<i64> = io::stdin()
        .lock()
        .lines()
        .next()
        .expect("No input")
        .unwrap()
        .split(',')
        .map(|w| w.parse::<i64>().expect("invalid int"))
        .collect();

    let mut mem = contents.clone();
    let mut pc = 0;

    loop {
        let fun = decode(&mem, pc);

        if fun(&mut mem, &mut pc) {
            break;
        }
    }
}
