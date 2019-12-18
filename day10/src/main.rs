use std::cmp::Ordering;
use std::f64;
use std::io::{self, BufRead};

fn gcd(mut x: i64, mut y: i64) -> i64 {
    if x < 0 {
        gcd(-x, y)
    } else if y < 0 {
        gcd(x, -y)
    } else {
        while y > 0 {
            let t = y;
            y = x % y;
            x = t;
        }
        x
    }
}

fn main() {
    let asteroids: Vec<(usize, usize)> = io::stdin()
        .lock()
        .lines()
        .enumerate()
        .flat_map(|(y, row)| -> Vec<(usize, usize)> {
            let s = row.unwrap();
            s.chars()
                .enumerate()
                .filter(|&(_, c)| c == '#')
                .map(|(x, _)| (y, x))
                .collect()
        })
        .collect();

    let mut best_vectors = Vec::new();
    let mut best_observable = 0;
    let mut base_y = 0;
    let mut base_x = 0;

    for &base in &asteroids {
        let vectors: Vec<(i64, i64, i64)> = asteroids
            .iter()
            .filter(|&other| base != *other)
            .map(|&other| {
                (
                    other.0 as i64 - base.0 as i64,
                    other.1 as i64 - base.1 as i64,
                )
            })
            .map(|(y, x)| {
                let k = gcd(y, x);
                (y / k, x / k, k)
            })
            .collect();

        let mut directions: Vec<(i64, i64)> = vectors.iter().map(|&(y, x, _)| (y, x)).collect();

        directions.sort();
        directions.dedup();

        let num_observable = directions.len();
        if num_observable > best_observable {
            best_observable = num_observable;
            best_vectors = vectors;
            base_y = base.0;
            base_x = base.1;
        }
    }

    println!("Base is at {},{}", base_x, base_y);

    let mut radial: Vec<(f64, i64, i64, i64)> = best_vectors
        .iter()
        .map(|(y_dir, x_dir, k)| {
            let mut angle = (*x_dir as f64).atan2(-*y_dir as f64);
            if angle < 0.0 {
                angle += 2.0 * f64::consts::PI;
            }
            let y = y_dir * k + base_y as i64;
            let x = x_dir * k + base_x as i64;

            (angle, *k, y, x)
        })
        .collect();

    radial.sort_by(
        |(a1, k1, _, _), (a2, k2, _, _)| match a1.partial_cmp(a2).unwrap() {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => k1.cmp(k2),
        },
    );

    let mut prev: Option<f64> = None;
    let mut dup_count: usize = 0;

    let mut rot_count: Vec<(usize, i64, i64)> = radial
        .iter()
        .map(|(a, _, y, x)| {
            match prev {
                None => {
                    dup_count = 0;
                    prev = Some(*a);
                }
                Some(pa) => {
                    if a.eq(&pa) {
                        dup_count += 1;
                    } else {
                        dup_count = 0;
                        prev = Some(*a);
                    }
                }
            };
            (dup_count, *y, *x)
        })
        .collect();

    rot_count.sort_by_key(|(k, _, _)| *k);

    let (_, y, x) = rot_count[199];

    println!("{}", x * 100 + y);
}
