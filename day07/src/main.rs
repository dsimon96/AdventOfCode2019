use std::cmp::max;
use std::io::{self, BufRead};

use permutohedron::LexicalPermutation;

struct Context {
    mem: Vec<i64>,
    pc: usize,
    input: Vec<i64>,
    in_idx: usize,
    output: Vec<i64>
}

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

fn add(ctx: &mut Context) -> bool {
    let (lhs, rhs, out) = get_args(&ctx.mem, ctx.pc);

    ctx.mem[out as usize] = lhs + rhs;
    ctx.pc += 4;

    false
}

fn mul(ctx: &mut Context) -> bool {
    let (lhs, rhs, out) = get_args(&ctx.mem, ctx.pc);

    ctx.mem[out as usize] = lhs * rhs;
    ctx.pc += 4;

    false
}

fn get_input(ctx: &mut Context) -> i64 {
    if ctx.in_idx < ctx.input.len() {
        let res = ctx.input[ctx.in_idx];
        ctx.in_idx += 1;

        res
    } else {
        io::stdin()
            .lock()
            .lines()
            .next()
            .expect("No input")
            .unwrap()
            .parse::<i64>()
            .expect("invalid int")
    }
}

fn input(ctx: &mut Context) -> bool {
    let out = ctx.mem[ctx.pc + 1];

    ctx.mem[out as usize] = get_input(ctx);
    ctx.pc += 2;

    false
}

fn output(ctx: &mut Context) -> bool {
    let val = get_arg(&ctx.mem, ctx.pc, 1);

    ctx.output.push(val);
    ctx.pc += 2;

    false
}

fn jump_if_true(ctx: &mut Context) -> bool {
    let (lhs, rhs) = (get_arg(&ctx.mem, ctx.pc, 1), get_arg(&ctx.mem, ctx.pc, 2));

    if lhs != 0 {
        ctx.pc = rhs as usize;
    } else {
        ctx.pc += 3;
    }

    false
}

fn jump_if_false(ctx: &mut Context) -> bool {
    let (lhs, rhs) = (get_arg(&ctx.mem, ctx.pc, 1), get_arg(&ctx.mem, ctx.pc, 2));

    if lhs == 0 {
        ctx.pc = rhs as usize;
    } else {
        ctx.pc += 3;
    }

    false
}

fn less_than(ctx: &mut Context) -> bool {
    let (lhs, rhs, out) = get_args(&ctx.mem, ctx.pc);

    if lhs < rhs {
        ctx.mem[out as usize] = 1;
    } else {
        ctx.mem[out as usize] = 0;
    }
    ctx.pc += 4;

    false
}

fn equals(ctx: &mut Context) -> bool {
    let (lhs, rhs, out) = get_args(&ctx.mem, ctx.pc);

    if lhs == rhs {
        ctx.mem[out as usize] = 1;
    } else {
        ctx.mem[out as usize] = 0;
    }
    ctx.pc += 4;

    false
}

fn ret(_ctx: &mut Context) -> bool {
    true
}

fn decode(contents: &Vec<i64>, pc: usize) -> fn(&mut Context) -> bool {
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

fn run_for_result(prog: &Vec<i64>, input: Vec<i64>) -> Vec<i64> {
    let mut ctx = Context {
        mem: prog.clone(),
        pc: 0,
        input,
        in_idx: 0,
        output: Vec::new()
    };

    loop {
        let fun = decode(&ctx.mem, ctx.pc);

        if fun(&mut ctx) {
            break ctx.output
        }
    }
}

fn main() {
    let prog: Vec<i64> = io::stdin()
        .lock()
        .lines()
        .next()
        .expect("No input")
        .unwrap()
        .split(',')
        .map(|w| w.parse::<i64>().expect("invalid int"))
        .collect();

    let mut best: i64 = 0;
    let mut data = vec!(5, 6, 7, 8, 9);

    loop {
        let mut signal: i64 = 0;
        for amp in 0..5 {
            let phase = data[amp];
            let input = vec![phase, signal];

            signal = run_for_result(&prog, input)[0];
        }
        best = max(best, signal);

        if !data.next_permutation() {
            break;
        }
    }

    println!("{}", best);
}
