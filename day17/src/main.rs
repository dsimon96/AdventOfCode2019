use std::collections::{HashMap, VecDeque};
use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Index, IndexMut};

const PAGE_SIZE: usize = 32768;

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

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
enum Turn {
    Left,
    Right,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Inst {
    turn: Turn,
    num: usize,
}

fn parse_img(ctx: &mut Context) -> Vec<Vec<i64>> {
    let mut img: Vec<Vec<i64>> = Vec::new();
    let mut row: Vec<i64> = Vec::new();

    while let Some(val) = ctx.output.pop_front() {
        match val {
            10 => {
                img.push(row);
                row = Vec::new();
            }
            _ => row.push(val),
        };
    }

    assert!(row.is_empty());

    img
}

fn alignment(img: &[Vec<i64>]) -> usize {
    let num_rows = img.len() - 1;
    let num_cols = img[0].len();

    let mut sum = 0;

    for row in 0..num_rows {
        for col in 0..num_cols {
            if img[row][col] == 35
                && (row == 0 || img[row - 1][col] == 35)
                && (row + 1 == num_rows || img[row + 1][col] == 35)
                && (col == 0 || img[row][col - 1] == 35)
                && (col + 1 == num_cols || img[row][col + 1] == 35)
            {
                sum += row * col;
            }
        }
    }

    sum
}

fn get_robot_pos(img: &[Vec<i64>]) -> (usize, usize, Direction) {
    for (row, line) in img.iter().enumerate() {
        for (col, c) in line.iter().enumerate() {
            match c {
                60 => return (row, col, Direction::West),
                62 => return (row, col, Direction::East),
                94 => return (row, col, Direction::North),
                118 => return (row, col, Direction::South),
                _ => (),
            }
        }
    }

    unreachable!()
}

fn try_move(robot: &(usize, usize, Direction), img: &[Vec<i64>]) -> Option<(usize, usize)> {
    let num_rows = img.len() - 1;
    let num_cols = img[0].len();

    match robot.2 {
        Direction::North => {
            if robot.0 > 0 {
                Some((robot.0 - 1, robot.1))
            } else {
                None
            }
        }
        Direction::South => {
            if robot.0 < num_rows - 1 {
                Some((robot.0 + 1, robot.1))
            } else {
                None
            }
        }
        Direction::West => {
            if robot.1 > 0 {
                Some((robot.0, robot.1 - 1))
            } else {
                None
            }
        }
        Direction::East => {
            if robot.1 < num_cols - 1 {
                Some((robot.0, robot.1 + 1))
            } else {
                None
            }
        }
    }
}

fn construct_path(img: &[Vec<i64>]) -> Vec<Inst> {
    let num_rows = img.len() - 1;
    let num_cols = img[0].len();

    let mut path: Vec<Inst> = Vec::new();

    let mut robot: (usize, usize, Direction) = get_robot_pos(img);
    let mut move_count = 0;
    let mut prev_turn: Turn = Turn::Left;

    loop {
        // try to move forward
        let new_pos: Option<(usize, usize)> = try_move(&robot, img);
        if new_pos.is_some() {
            let (nr, nc) = new_pos.unwrap();
            if img[nr][nc] == 35 {
                robot = (nr, nc, robot.2);
                move_count += 1;
                continue;
            }
        }

        // unable to move forward
        if move_count > 0 {
            path.push(Inst {
                turn: prev_turn,
                num: move_count,
            });
        }
        move_count = 0;

        // try to turn
        match robot.2 {
            Direction::North => {
                // can we turn left?
                if robot.1 > 0 && img[robot.0][robot.1 - 1] == 35 {
                    prev_turn = Turn::Left;
                    robot = (robot.0, robot.1, Direction::West);
                    continue;
                }

                // can we turn right?
                if robot.1 < num_cols - 1 && img[robot.0][robot.1 + 1] == 35 {
                    prev_turn = Turn::Right;
                    robot = (robot.0, robot.1, Direction::East);
                    continue;
                }
            }
            Direction::South => {
                // can we turn left?
                if robot.1 < num_cols - 1 && img[robot.0][robot.1 + 1] == 35 {
                    prev_turn = Turn::Left;
                    robot = (robot.0, robot.1, Direction::East);
                    continue;
                }

                // can we turn right?
                if robot.1 > 0 && img[robot.0][robot.1 - 1] == 35 {
                    prev_turn = Turn::Right;
                    robot = (robot.0, robot.1, Direction::West);
                    continue;
                }
            }
            Direction::East => {
                // can we turn left?
                if robot.0 > 0 && img[robot.0 - 1][robot.1] == 35 {
                    prev_turn = Turn::Left;
                    robot = (robot.0, robot.1, Direction::North);
                    continue;
                }

                // can we turn right?
                if robot.0 < num_rows - 1 && img[robot.0 + 1][robot.1] == 35 {
                    prev_turn = Turn::Right;
                    robot = (robot.0, robot.1, Direction::South);
                    continue;
                }
            }
            Direction::West => {
                // can we turn left?
                if robot.0 < num_rows - 1 && img[robot.0 + 1][robot.1] == 35 {
                    prev_turn = Turn::Left;
                    robot = (robot.0, robot.1, Direction::South);
                    continue;
                }

                // can we turn right?
                if robot.0 > 0 && img[robot.0 - 1][robot.1] == 35 {
                    prev_turn = Turn::Right;
                    robot = (robot.0, robot.1, Direction::North);
                    continue;
                }
            }
        }
        break;
    }

    path
}

fn valid_len(seq: &[Inst]) -> bool {
    let len: usize = seq
        .iter()
        .map(|i| -> usize { 2 + i.num.to_string().len() })
        .sum();

    len + seq.len() - 1 <= 20
}

#[derive(Debug)]
enum Pattern {
    A,
    B,
    C,
}

fn serialize_seq(inst: &[Inst]) -> String {
    inst.iter()
        .map(|i| match i.turn {
            Turn::Left => format!("L,{}", i.num),
            Turn::Right => format!("R,{}", i.num),
        })
        .collect::<Vec<String>>()
        .join(",")
}

fn to_ascii(c: char) -> i64 {
    let mut buf: [u8; 1] = [0; 1];

    assert!(c.len_utf8() == 1);

    c.encode_utf8(&mut buf);

    i64::from(buf[0])
}

fn process_output(ctx: &mut Context) -> (String, String, String, String) {
    let img: Vec<Vec<i64>> = parse_img(ctx);

    println!("{}", alignment(&img));

    let path: Vec<Inst> = construct_path(&img);

    let prefixes: Vec<Vec<Inst>> = (1..=path.len())
        .map(|n| path[..n].to_vec())
        .filter(|v| valid_len(v))
        .collect();

    let suffixes: Vec<Vec<Inst>> = (0..path.len())
        .map(|n| path[n..].to_vec())
        .filter(|v| valid_len(v))
        .collect();

    for prefix in &prefixes {
        for suffix in &suffixes {
            let mut i = 0;
            let mut start = 0;
            let mut end = 0;

            let mut inst: Vec<Pattern> = Vec::new();
            let mut between: Vec<Vec<Inst>> = Vec::new();

            while i < path.len() {
                let chars_remaining = path.len() - i;
                if prefix.len() <= chars_remaining
                    && prefix.iter().zip(path[i..].iter()).all(|(x, y)| x == y)
                {
                    if start != end {
                        inst.push(Pattern::B);
                        between.push(path[start..end].to_vec());
                    }
                    inst.push(Pattern::A);
                    i += prefix.len();
                    start = i;
                    end = i;
                } else if suffix.len() <= chars_remaining
                    && suffix.iter().zip(path[i..].iter()).all(|(x, y)| x == y)
                {
                    if start != end {
                        inst.push(Pattern::B);
                        between.push(path[start..end].to_vec());
                    }
                    inst.push(Pattern::C);
                    i += suffix.len();
                    start = i;
                    end = i;
                } else {
                    i += 1;
                    end += 1;
                }
            }

            let b = between.get(0).unwrap();

            if end != path.len() || !between[1..].iter().all(|v| v == b) {
                continue;
            }

            // Found working pattern
            let serialized_inst = inst
                .into_iter()
                .map(|p| match p {
                    Pattern::A => String::from("A"),
                    Pattern::B => String::from("B"),
                    Pattern::C => String::from("C"),
                })
                .collect::<Vec<String>>()
                .join(",");

            return (serialized_inst, serialize_seq(prefix), serialize_seq(&b), serialize_seq(suffix))
        }
    }

    unreachable!()
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

    let mut ctx = Context {
        mem: init_vmem(&prog),
        pc: 0,
        status: Status::Running,
        input: VecDeque::new(),
        output: VecDeque::new(),
        rel_base: 0,
    };

    run(&mut ctx);
    let (inst, a, b, c) = process_output(&mut ctx);

    // re-initialize
    ctx = Context {
        mem: init_vmem(&prog),
        pc: 0,
        status: Status::Running,
        input: VecDeque::new(),
        output: VecDeque::new(),
        rel_base: 0,
    };
    ctx.mem[0] = 2;

    for ch in inst.chars() {
        add_input(&mut ctx, to_ascii(ch));
    }
    add_input(&mut ctx, to_ascii('\n'));

    for ch in a.chars() {
        add_input(&mut ctx, to_ascii(ch));
    }
    add_input(&mut ctx, to_ascii('\n'));

    for ch in b.chars() {
        add_input(&mut ctx, to_ascii(ch));
    }
    add_input(&mut ctx, to_ascii('\n'));

    for ch in c.chars() {
        add_input(&mut ctx, to_ascii(ch));
    }
    add_input(&mut ctx, to_ascii('\n'));

    add_input(&mut ctx, to_ascii('n'));
    add_input(&mut ctx, to_ascii('\n'));

    while ctx.status != Status::Halted {
        run(&mut ctx);
        if ctx.status == Status::WaitingForInput {
            unreachable!()
        }
    }

    while let Some(val) = ctx.output.pop_front() {
        if val < 256 { continue; }
        println!("{}", val);
    }
}
