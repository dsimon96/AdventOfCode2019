use std::collections::{HashMap, HashSet, VecDeque};
use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Index, IndexMut};

use num_derive::ToPrimitive;
use num_traits::cast::ToPrimitive;

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

#[derive(ToPrimitive, Debug, Clone, Copy)]
enum Direction {
    Start = 0,
    North = 1,
    South = 2,
    West = 3,
    East = 4,
}

fn opposite(dir: Direction) -> Direction {
    match dir {
        Direction::North => Direction::South,
        Direction::South => Direction::North,
        Direction::West => Direction::East,
        Direction::East => Direction::West,
        _ => unreachable!(),
    }
}

fn do_move(pos: (i64, i64), dir: Direction) -> (i64, i64) {
    match dir {
        Direction::North => (pos.0, pos.1 - 1),
        Direction::South => (pos.0, pos.1 + 1),
        Direction::East => (pos.0 + 1, pos.1),
        Direction::West => (pos.0 - 1, pos.1),
        _ => unreachable!(),
    }
}

struct Droid {
    reachable: HashMap<(i64, i64), bool>,
    seen: HashSet<(i64, i64)>,
    to_explore: VecDeque<(i64, i64)>,
    cur: (i64, i64),
    path: VecDeque<Direction>,
    oxy: Option<(i64, i64)>,
}

fn explore(pos: (i64, i64)) -> VecDeque<(i64, i64)> {
    let mut result = VecDeque::new();

    for dir in &[
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ] {
        result.push_back(do_move(pos, *dir));
    }

    result
}

fn plan_path(droid: &Droid, tgt: (i64, i64)) -> VecDeque<Direction> {
    let mut visited: HashMap<(i64, i64), Direction> = HashMap::new();
    let mut queue: VecDeque<(i64, i64)> = VecDeque::new();
    let mut result = VecDeque::new();

    visited.insert(droid.cur, Direction::Start);
    queue.push_back(droid.cur);

    while let Some(pos) = queue.pop_front() {
        if pos == tgt {
            let mut cur = pos;
            while cur != droid.cur {
                let cur_dir = visited.get(&cur).unwrap();
                result.push_front(*cur_dir);
                cur = do_move(cur, opposite(*cur_dir));
            }
            break;
        }

        for dir in &[
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ] {
            let next = do_move(pos, *dir);
            if !droid.seen.contains(&next) {
                continue;
            }
            let reachable =
                droid.reachable.contains_key(&next) && *droid.reachable.get(&next).unwrap();

            if next == tgt || reachable && !visited.contains_key(&next) {
                visited.insert(next, *dir);
                queue.push_back(next);
            }
        }
    }

    result
}

fn time_floodfill(droid: &mut Droid) -> usize {
    let mut filled: HashMap<(i64, i64), usize> = HashMap::new();
    let mut to_prop: VecDeque<(i64, i64)> = VecDeque::new();

    filled.insert(droid.oxy.unwrap(), 0);
    to_prop.push_back(droid.oxy.unwrap());

    let mut max: usize = 0;

    while let Some(pos) = to_prop.pop_front() {
        let next_val = filled[&pos] + 1;
        for next in explore(pos) {
            if droid.reachable[&next] && !filled.contains_key(&next) {
                max = next_val;
                filled.insert(next, next_val);
                to_prop.push_back(next);
            }
        }
    }

    max
}

fn process_output(ctx: &mut Context, droid: &mut Droid) -> Option<usize> {
    while droid.path.len() > 1 {
        let val = ctx.output.pop_front().unwrap();
        assert!(val != 0);
        let next_move = droid.path.pop_front().unwrap();

        droid.cur = do_move(droid.cur, next_move);
    }
    let last_move = droid.path.pop_front().unwrap();
    let pos = do_move(droid.cur, last_move);
    match ctx.output.pop_front().unwrap() {
        0 => {
            droid.reachable.insert(pos, false);

            if droid.to_explore.is_empty() {
                return Some(time_floodfill(droid));
            }

            let next = droid.to_explore.pop_back().unwrap();;
            droid.path = plan_path(droid, next);
            for dir in &droid.path {
                add_input(ctx, dir.to_i64().unwrap());
            }

            None
        }
        1 => {
            droid.cur = pos;
            droid.reachable.insert(droid.cur, true);

            for next in explore(pos) {
                if !droid.seen.contains(&next) {
                    droid.seen.insert(next);
                    droid.to_explore.push_back(next);
                }
            }

            if droid.to_explore.is_empty() {
                return Some(time_floodfill(droid));
            }

            let next = droid.to_explore.pop_back().unwrap();
            droid.path = plan_path(droid, next);
            for dir in &droid.path {
                add_input(ctx, dir.to_i64().unwrap());
            }

            None
        }
        2 => {
            droid.cur = do_move(droid.cur, last_move);
            droid.reachable.insert(droid.cur, true);
            droid.oxy = Some(droid.cur);

            // part 1
            // Some(plan_path(droid, (0, 0)).len())
            for next in explore(pos) {
                if !droid.seen.contains(&next) {
                    droid.seen.insert(next);
                    droid.to_explore.push_back(next);
                }
            }

            if droid.to_explore.is_empty() {
                return Some(time_floodfill(droid));
            }

            let next = droid.to_explore.pop_back().unwrap();
            droid.path = plan_path(droid, next);
            for dir in &droid.path {
                add_input(ctx, dir.to_i64().unwrap());
            }

            None
        }
        _ => unreachable!(),
    }
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

    let mut droid = Droid {
        reachable: HashMap::new(),
        seen: HashSet::new(),
        to_explore: VecDeque::new(),
        cur: (0, 0),
        path: VecDeque::new(),
        oxy: None,
    };

    droid.seen.insert((0, 0));
    droid.seen.insert((0, -1));
    droid.seen.insert((0, 1));
    droid.seen.insert((1, 0));
    droid.seen.insert((-1, 0));
    droid.reachable.insert((0, 0), true);

    droid.to_explore.push_back((0, 1));
    droid.to_explore.push_back((1, 0));
    droid.to_explore.push_back((-1, 0));

    droid.path = VecDeque::new();
    droid.path.push_back(Direction::North);

    let mut ctx = Context {
        mem: init_vmem(&prog),
        pc: 0,
        status: Status::Running,
        input: VecDeque::new(),
        output: VecDeque::new(),
        rel_base: 0,
    };
    add_input(&mut ctx, Direction::North.to_i64().unwrap());
    while ctx.status != Status::Halted {
        run(&mut ctx);
        if let Some(dist) = process_output(&mut ctx, &mut droid) {
            println!("{}", dist);
            return;
        }
    }
}
