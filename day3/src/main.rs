use std::cmp::min;
use std::collections::HashMap;
use std::io::{self, BufRead};

enum Direction {
    R,
    L,
    U,
    D,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Point {
    x: i64,
    y: i64,
}

fn do_move(p: Point, d: &Direction) -> Point {
    match d {
        Direction::R => Point { x: p.x + 1, y: p.y },
        Direction::L => Point { x: p.x - 1, y: p.y },
        Direction::U => Point { x: p.x, y: p.y + 1 },
        Direction::D => Point { x: p.x, y: p.y - 1 },
    }
}

fn main() {
    let mut prev: Point = Point { x: 0, y: 0 };
    let mut step = 0;

    let stdin = io::stdin();
    let mut iter = stdin.lock().lines();

    let line1 = iter.next().unwrap().unwrap();
    let line2 = iter.next().unwrap().unwrap();

    let mut pos1: HashMap<Point, usize> = HashMap::new();
    for mvmt in line1.split(",") {
        let dir = match mvmt.chars().next().unwrap() {
            'R' => Direction::R,
            'L' => Direction::L,
            'U' => Direction::U,
            'D' => Direction::D,
            _ => panic!("Invalid direction"),
        };

        let len = mvmt
            .get(1..)
            .unwrap()
            .parse::<usize>()
            .expect("invalid len");

        for _ in 0..len {
            let next = do_move(prev, &dir);
            step += 1;

            pos1.insert(next.clone(), step);

            prev = next;
        }
    }

    prev = Point { x: 0, y: 0 };
    step = 0;
    let mut pos2: HashMap<Point, usize> = HashMap::new();
    for mvmt in line2.split(",") {
        let dir = match mvmt.chars().next().unwrap() {
            'R' => Direction::R,
            'L' => Direction::L,
            'U' => Direction::U,
            'D' => Direction::D,
            _ => panic!("Invalid direction"),
        };

        let len = mvmt
            .get(1..)
            .unwrap()
            .parse::<usize>()
            .expect("invalid len");

        for _ in 0..len {
            let next = do_move(prev, &dir);
            step += 1;

            pos2.insert(next.clone(), step);

            prev = next;
        }
    }

    let mut min_dist = std::u64::MAX;
    for point in pos1.keys() {
        let steps1 = *pos1.get(point).unwrap() as u64;
        if pos2.contains_key(point) {
            let steps2 = *pos2.get(point).unwrap() as u64;
            let dist = steps1 + steps2;

            println!("{:?}: {}", point, dist);

            min_dist = min(min_dist, dist);
        }
    }

    println!("{}", min_dist);
}
