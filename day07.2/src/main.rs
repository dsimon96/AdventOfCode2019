use std::cmp::max;
use std::collections::VecDeque;
use std::io::{self, BufRead};

use permutohedron::LexicalPermutation;

#[derive(PartialEq)]
enum Status {
    Running,
    WaitingForInput,
    Halted,
}

struct Context {
    mem: Vec<i64>,
    pc: usize,
    status: Status,
    input: VecDeque<i64>,
    output: VecDeque<i64>,
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

fn add(ctx: &mut Context) {
    let (lhs, rhs, out) = get_args(&ctx.mem, ctx.pc);

    ctx.mem[out as usize] = lhs + rhs;
    ctx.pc += 4;
}

fn mul(ctx: &mut Context) {
    let (lhs, rhs, out) = get_args(&ctx.mem, ctx.pc);

    ctx.mem[out as usize] = lhs * rhs;
    ctx.pc += 4;
}

fn input(ctx: &mut Context) {
    let out = ctx.mem[ctx.pc + 1];

    match ctx.input.pop_front() {
        Some(val) => {
            ctx.mem[out as usize] = val;
            ctx.pc += 2;
        }
        None => ctx.status = Status::WaitingForInput,
    }
}

fn output(ctx: &mut Context) {
    let val = get_arg(&ctx.mem, ctx.pc, 1);

    ctx.output.push_back(val);
    ctx.pc += 2;
}

fn jump_if_true(ctx: &mut Context) {
    let (lhs, rhs) = (get_arg(&ctx.mem, ctx.pc, 1), get_arg(&ctx.mem, ctx.pc, 2));

    if lhs != 0 {
        ctx.pc = rhs as usize;
    } else {
        ctx.pc += 3;
    }
}

fn jump_if_false(ctx: &mut Context) {
    let (lhs, rhs) = (get_arg(&ctx.mem, ctx.pc, 1), get_arg(&ctx.mem, ctx.pc, 2));

    if lhs == 0 {
        ctx.pc = rhs as usize;
    } else {
        ctx.pc += 3;
    }
}

fn less_than(ctx: &mut Context) {
    let (lhs, rhs, out) = get_args(&ctx.mem, ctx.pc);

    if lhs < rhs {
        ctx.mem[out as usize] = 1;
    } else {
        ctx.mem[out as usize] = 0;
    }
    ctx.pc += 4;
}

fn equals(ctx: &mut Context) {
    let (lhs, rhs, out) = get_args(&ctx.mem, ctx.pc);

    if lhs == rhs {
        ctx.mem[out as usize] = 1;
    } else {
        ctx.mem[out as usize] = 0;
    }
    ctx.pc += 4;
}

fn ret(ctx: &mut Context) {
    ctx.status = Status::Halted;
}

fn decode(contents: &Vec<i64>, pc: usize) -> fn(&mut Context) {
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

fn run(ctx: &mut Context) {
    while ctx.status == Status::Running {
        let fun = decode(&ctx.mem, ctx.pc);
        fun(ctx);
    }
}

fn add_input(ctx: &mut Context, val: i64) {
    ctx.input.push_back(val);
    if ctx.status == Status::WaitingForInput {
        ctx.status = Status::Running;
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
    let mut data = vec![5, 6, 7, 8, 9];

    loop {
        let mut amps: Vec<Context> = Vec::new();
        for idx in 0..5 {
            let mut ctx = Context {
                mem: prog.clone(),
                pc: 0,
                status: Status::Running,
                input: VecDeque::new(),
                output: VecDeque::new(),
            };
            add_input(&mut ctx, data[idx]);
            amps.push(ctx);
        }
        amps[0].output.push_back(0);

        let mut last_out = 0;
        while amps.iter().any(|ctx| ctx.status != Status::Halted) {
            for idx in 0..5 {
                run(&mut amps[idx]);

                while let Some(val) = amps[idx].output.pop_front() {
                    last_out = val;
                    add_input(&mut amps[(idx + 1) % 5], val);
                }
            }
        }

        best = max(best, last_out);
        if !data.next_permutation() {
            break;
        }
    }

    println!("{}", best);
}
