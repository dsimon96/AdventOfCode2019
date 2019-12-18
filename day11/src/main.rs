use std::collections::{HashMap, VecDeque};
use std::io::{self, BufRead};
use std::ops::{Index, IndexMut};

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::cast::{FromPrimitive, ToPrimitive};

const PAGE_SIZE: usize = 32768;
const MARGIN: i64 = 1;

#[derive(PartialEq)]
enum Status {
    Running,
    WaitingForInput,
    Halted,
}

type Page = [i64; PAGE_SIZE];
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

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(FromPrimitive, ToPrimitive)]
enum Color {
    Black = 0,
    White = 1,
}

struct Robot {
    dir: Direction,
    pos_x: i64,
    pos_y: i64,
    surface: HashMap<(i64, i64), Color>,
}

#[derive(FromPrimitive)]
enum Turn {
    Left = 0,
    Right = 1,
}

fn do_paint(robot: &mut Robot, color: Color) {
    robot.surface.insert((robot.pos_x, robot.pos_y), color);
}

fn do_turn(robot: &mut Robot, turn: Turn) {
    robot.dir = match (&robot.dir, turn) {
        (Direction::Up, Turn::Left) => Direction::Left,
        (Direction::Up, Turn::Right) => Direction::Right,
        (Direction::Right, Turn::Left) => Direction::Up,
        (Direction::Right, Turn::Right) => Direction::Down,
        (Direction::Down, Turn::Left) => Direction::Right,
        (Direction::Down, Turn::Right) => Direction::Left,
        (Direction::Left, Turn::Left) => Direction::Down,
        (Direction::Left, Turn::Right) => Direction::Up,
    };
}

fn do_move(robot: &mut Robot) {
    match &robot.dir {
        Direction::Up => {
            robot.pos_y += 1;
        }
        Direction::Right => {
            robot.pos_x += 1;
        }
        Direction::Down => {
            robot.pos_y -= 1;
        }
        Direction::Left => {
            robot.pos_x -= 1;
        }
    };
}

fn process_output(ctx: &mut Context, robot: &mut Robot) {
    while let Some(c_val) = ctx.output.pop_front() {
        let t_val = ctx.output.pop_front().expect("No turn specified!");
        let color = Color::from_i64(c_val).expect("Invalid color!");
        let turn = Turn::from_i64(t_val).expect("Invalid turn!");

        do_paint(robot, color);
        do_turn(robot, turn);
        do_move(robot);
    }
}

fn read_camera(robot: &Robot) -> i64 {
    robot
        .surface
        .get(&(robot.pos_x, robot.pos_y))
        .unwrap_or(&Color::Black)
        .to_i64()
        .unwrap()
}

fn visualize(surface: &HashMap<(i64, i64), Color>) {
    let xs: Vec<i64> = surface.keys().map(|&(x, _)| x).collect();
    let ys: Vec<i64> = surface.keys().map(|&(_, y)| y).collect();

    let top = ys.iter().max().unwrap() + MARGIN;
    let bottom = ys.iter().min().unwrap() - MARGIN;
    let left = xs.iter().min().unwrap() - MARGIN;
    let right = xs.iter().max().unwrap() + MARGIN;

    let num_rows = top - bottom + 1;
    let num_cols = right - left + 1;

    for row in 0..num_rows {
        let y = top - row;
        for col in 0..num_cols {
            let x = left + col;

            let color = surface.get(&(x,y)).unwrap_or(&Color::Black);

            // colors inverted for readability
            match color {
                Color::Black => print!("█"),
                Color::White => print!(" "),
            }
        }
        println!();
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

    let mut robot = Robot {
        dir: Direction::Up,
        pos_x: 0,
        pos_y: 0,
        surface: HashMap::new(),
    };
    robot.surface.insert((0,0), Color::White);

    let mut ctx = Context {
        mem: init_vmem(&prog),
        pc: 0,
        status: Status::Running,
        input: VecDeque::new(),
        output: VecDeque::new(),
        rel_base: 0,
    };
    while ctx.status != Status::Halted {
        run(&mut ctx);
        process_output(&mut ctx, &mut robot);
        if ctx.status == Status::WaitingForInput {
            add_input(&mut ctx, read_camera(&robot));
        }
    }

    visualize(&robot.surface);
}
