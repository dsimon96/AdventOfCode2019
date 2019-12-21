use std::collections::{HashMap, VecDeque};
use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Index, IndexMut};

const PART: usize = 2;
const PAGE_SIZE: usize = 32768;
const SANTA_SIZE: usize = 100;

#[derive(Clone, PartialEq)]
enum Status {
    Running,
    WaitingForInput,
    Halted,
}

type Page = [i64; PAGE_SIZE];
#[derive(Clone)]
struct Vmem {
    table: HashMap<usize, Page>,
}

impl Index<usize> for Vmem {
    type Output = i64;

    fn index(&self, idx: usize) -> &Self::Output {
        let page_num = idx / PAGE_SIZE;
        let page_off = idx % PAGE_SIZE;

        &self.table[&page_num][page_off]
    }
}

impl IndexMut<usize> for Vmem {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        let page_num = idx / PAGE_SIZE;
        let page_off = idx % PAGE_SIZE;

        &mut self.table.entry(page_num).or_insert([0; PAGE_SIZE])[page_off]
    }
}

fn init_vmem(prog: &[i64]) -> Vmem {
    let mut res = Vmem {
        table: HashMap::new(),
    };

    for i in 0..prog.len() {
        res[i] = prog[i];
    }

    res
}

#[derive(Clone)]
struct Context {
    mem: Vmem,
    pc: usize,
    status: Status,
    input: VecDeque<i64>,
    output: VecDeque<i64>,
    rel_base: i64,
}

fn get_arg(ctx: &Context, offset: usize) -> i64 {
    let opcode = ctx.mem[ctx.pc] as usize;
    let mode = opcode / 10usize.pow(1 + offset as u32) % 10;

    match mode {
        0 => {
            let pos = ctx.mem[ctx.pc + offset] as usize;
            ctx.mem[pos]
        }
        1 => ctx.mem[ctx.pc + offset],
        2 => {
            let rel = ctx.mem[ctx.pc + offset];
            ctx.mem[(ctx.rel_base + rel) as usize]
        }
        _ => panic!("Invalid mode"),
    }
}

fn get_out(ctx: &mut Context, offset: usize) -> &mut i64 {
    let opcode = ctx.mem[ctx.pc] as usize;
    let mode = opcode / 10usize.pow(1 + offset as u32) % 10;

    match mode {
        0 => {
            let pos = ctx.mem[ctx.pc + offset] as usize;
            &mut ctx.mem[pos]
        }
        1 => &mut ctx.mem[ctx.pc + offset],
        2 => {
            let rel = ctx.mem[ctx.pc + offset];
            &mut ctx.mem[(ctx.rel_base + rel) as usize]
        }
        _ => panic!("Invalid mode"),
    }
}

fn get_args(ctx: &mut Context) -> (i64, i64, &mut i64) {
    (get_arg(ctx, 1), get_arg(ctx, 2), get_out(ctx, 3))
}

fn add(ctx: &mut Context) {
    let (lhs, rhs, out) = get_args(ctx);

    *out = lhs + rhs;
    ctx.pc += 4;
}

fn mul(ctx: &mut Context) {
    let (lhs, rhs, out) = get_args(ctx);

    *out = lhs * rhs;
    ctx.pc += 4;
}

fn input(ctx: &mut Context) {
    if let Some(val) = ctx.input.pop_front() {
        let out = get_out(ctx, 1);
        *out = val;
        ctx.pc += 2;
    } else {
        ctx.status = Status::WaitingForInput;
    }
}

fn output(ctx: &mut Context) {
    let val = get_arg(ctx, 1);

    ctx.output.push_back(val);
    ctx.pc += 2;
}

fn jump_if_true(ctx: &mut Context) {
    let (lhs, rhs) = (get_arg(ctx, 1), get_arg(ctx, 2));

    if lhs != 0 {
        ctx.pc = rhs as usize;
    } else {
        ctx.pc += 3;
    }
}

fn jump_if_false(ctx: &mut Context) {
    let (lhs, rhs) = (get_arg(ctx, 1), get_arg(ctx, 2));

    if lhs == 0 {
        ctx.pc = rhs as usize;
    } else {
        ctx.pc += 3;
    }
}

fn less_than(ctx: &mut Context) {
    let (lhs, rhs, out) = get_args(ctx);

    if lhs < rhs {
        *out = 1;
    } else {
        *out = 0;
    }
    ctx.pc += 4;
}

fn equals(ctx: &mut Context) {
    let (lhs, rhs, out) = get_args(ctx);

    if lhs == rhs {
        *out = 1;
    } else {
        *out = 0;
    }
    ctx.pc += 4;
}

fn adjust_rel_base(ctx: &mut Context) {
    let adj = get_arg(ctx, 1);

    ctx.rel_base += adj;

    ctx.pc += 2;
}

fn ret(ctx: &mut Context) {
    ctx.status = Status::Halted;
}

fn decode(contents: &Vmem, pc: usize) -> fn(&mut Context) {
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
        9 => adjust_rel_base,
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

type Coords = (usize, usize);

fn check_coords(base: &Context, coords: Coords) -> bool {
    let mut ctx = base.clone();

    let (x, y) = coords;

    add_input(&mut ctx, x as i64);
    add_input(&mut ctx, y as i64);

    run(&mut ctx);

    ctx.output.pop_front().unwrap() == 1
}

fn main() {
    let prog_file = File::open(args().nth(1).expect("No program!")).unwrap();
    let prog: Vec<i64> = BufReader::new(prog_file)
        .lines()
        .next()
        .expect("No input")
        .unwrap()
        .split(',')
        .map(|w| w.parse::<i64>().expect("invalid int"))
        .collect();

    let ctx = Context {
        mem: init_vmem(&prog),
        pc: 0,
        status: Status::Running,
        input: VecDeque::new(),
        output: VecDeque::new(),
        rel_base: 0,
    };

    if PART == 1 {
        let mut count = 0;

        for y in 0..50 {
            for x in 0..50 {
                if check_coords(&ctx, (x, y)) {
                    print!("#");
                    count += 1;
                } else {
                    print!(".");
                }
            }
            println!();
        }

        println!("{}", count);
    } else {
        let mut x_left = 0;
        let mut y_bot = 99;
        loop {
            if check_coords(&ctx, (x_left, y_bot)) {
                let x_right = x_left + SANTA_SIZE - 1;
                let y_top = y_bot - (SANTA_SIZE - 1);
                if check_coords(&ctx, (x_right, y_top)) {
                    println!("{}", x_left * 10000 + y_top);
                    return;
                } else {
                    y_bot += 1;
                }
            } else {
                x_left += 1;
            }
        }
    }
}
