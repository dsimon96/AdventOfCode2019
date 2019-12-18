use std::collections::HashSet;
use std::io::{self, BufRead};
use std::ops::AddAssign;
use std::str::FromStr;

use lazy_static::lazy_static;
use num::Integer;
use regex::{Error, Regex};

const NUM_STEPS: usize = 1000;

#[derive(Debug)]
struct Vec3 {
    x: i64,
    y: i64,
    z: i64,
}

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"<x=(?P<x>-?\d+),\s*y=(?P<y>-?\d+),\s*z=(?P<z>-?\d+)>").unwrap();
}

type Comp = (i64, i64, i64, i64, i64, i64, i64, i64);

impl FromStr for Vec3 {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let caps = RE.captures(s).expect("Invalid Vec3!");

        Ok(Vec3 {
            x: caps
                .name("x")
                .unwrap()
                .as_str()
                .parse::<i64>()
                .expect("Invalid int!"),
            y: caps
                .name("y")
                .unwrap()
                .as_str()
                .parse::<i64>()
                .expect("Invalid int!"),
            z: caps
                .name("z")
                .unwrap()
                .as_str()
                .parse::<i64>()
                .expect("Invalid int!"),
        })
    }
}

impl<'a> AddAssign<&'a Vec3> for Vec3 {
    fn add_assign(&mut self, other: &'a Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

#[derive(Debug)]
struct Moon {
    pos: Vec3,
    vel: Vec3,
}

fn apply_grav_axis(pos1: i64, pos2: i64, vel1: &mut i64, vel2: &mut i64) {
    if pos1 > pos2 {
        *vel1 -= 1;
        *vel2 += 1;
    } else if pos1 < pos2 {
        *vel1 += 1;
        *vel2 -= 1;
    }
}

fn apply_gravity(moons: &mut Vec<Moon>) {
    for i in 0..moons.len() {
        let (m1, rem) = moons[i..].split_first_mut().unwrap();
        for m2 in rem {
            apply_grav_axis(m1.pos.x, m2.pos.x, &mut m1.vel.x, &mut m2.vel.x);
            apply_grav_axis(m1.pos.y, m2.pos.y, &mut m1.vel.y, &mut m2.vel.y);
            apply_grav_axis(m1.pos.z, m2.pos.z, &mut m1.vel.z, &mut m2.vel.z);
        }
    }
}

fn apply_velocity(moons: &mut Vec<Moon>) {
    for moon in moons {
        moon.pos += &moon.vel;
    }
}

fn do_simulation(moons: &mut Vec<Moon>) {
    apply_gravity(moons);
    apply_velocity(moons);
}

fn get_energy(vec: &Vec3) -> i64 {
    vec.x.abs() + vec.y.abs() + vec.z.abs()
}

fn total_energy(moons: &[Moon]) -> i64 {
    moons
        .iter()
        .map(|moon| get_energy(&moon.pos) * get_energy(&moon.vel))
        .sum()
}

fn check_repeat(idx: usize, m: &[Moon], set: &mut HashSet<Comp>) -> bool {
    let comp = match idx {
        0 => (
            m[0].pos.x, m[1].pos.x, m[2].pos.x, m[3].pos.x, m[0].vel.x, m[1].vel.x, m[2].vel.x,
            m[3].vel.x,
        ),
        1 => (
            m[0].pos.y, m[1].pos.y, m[2].pos.y, m[3].pos.y, m[0].vel.y, m[1].vel.y, m[2].vel.y,
            m[3].vel.y,
        ),
        2 => (
            m[0].pos.z, m[1].pos.z, m[2].pos.z, m[3].pos.z, m[0].vel.z, m[1].vel.z, m[2].vel.z,
            m[3].vel.z,
        ),
        _ => panic!("Invalid idx"),
    };

    !set.insert(comp)
}

fn main() {
    let stdin = io::stdin();

    let mut moons: Vec<Moon> = stdin
        .lock()
        .lines()
        .map(|line| Moon {
            pos: Vec3::from_str(&line.unwrap()).expect("Invalid Moon!"),
            vel: Vec3 { x: 0, y: 0, z: 0 },
        })
        .collect();

    let mut seen_x: HashSet<Comp> = HashSet::new();
    let mut seen_y: HashSet<Comp> = HashSet::new();
    let mut seen_z: HashSet<Comp> = HashSet::new();

    let mut x_rep: Option<usize> = None;
    let mut y_rep: Option<usize> = None;
    let mut z_rep: Option<usize> = None;

    let mut num_iters: usize = 0;
    loop {
        if x_rep.is_none() && check_repeat(0, &moons, &mut seen_x) {
            x_rep = Some(num_iters);
        }
        if y_rep.is_none() && check_repeat(1, &moons, &mut seen_y) {
            y_rep = Some(num_iters);
        }
        if z_rep.is_none() && check_repeat(2, &moons, &mut seen_z) {
            z_rep = Some(num_iters);
        }
        if x_rep.is_some() && y_rep.is_some() && z_rep.is_some() {
            break;
        }

        do_simulation(&mut moons);

        num_iters += 1;
        if num_iters == NUM_STEPS {
            println!("Kinetic energy after {} steps: {}", NUM_STEPS, total_energy(&moons));
        }
    }

    println!(
        "Num steps until repeat: {}",
        x_rep.unwrap().lcm(&y_rep.unwrap()).lcm(&z_rep.unwrap())
    );
    println!("(Simulated {} steps in total)", num_iters);
}
