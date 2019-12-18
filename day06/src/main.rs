use std::collections::HashMap;
use std::io::{self, BufRead};

fn count_orbits(
    orbits: &HashMap<String, String>,
    memo: &mut HashMap<String, usize>,
    k: &String,
) -> usize {
    if memo.contains_key(k) {
        *memo.get(k).unwrap()
    } else if !orbits.contains_key(k) {
        memo.insert(k.clone(), 0);
        0
    } else {
        let body = orbits.get(k).unwrap();
        let res = 1 + count_orbits(orbits, memo, body);
        memo.insert(k.clone(), res);
        res
    }
}

fn main() {
    let mut orbits: HashMap<String, String> = HashMap::new();

    for line in io::stdin().lock().lines() {
        let rel = line.unwrap();
        let (body, sat) = rel.split_at(rel.find(")").unwrap());

        orbits.insert(sat[1..].to_string(), body.to_string());
    }

    let mut depths: HashMap<String, usize> = HashMap::new();
    let result: usize = orbits
        .keys()
        .map(|k| count_orbits(&orbits, &mut depths, k))
        .sum();

    println!("Part 1: {}", result);

    let mut my_location = orbits.get("YOU").unwrap();
    let mut santa_location = orbits.get("SAN").unwrap();

    let mut my_depth = *depths.get(my_location).unwrap();
    let mut santa_depth = *depths.get(santa_location).unwrap();

    let mut total = 0;

    while my_depth > santa_depth {
        my_location = orbits.get(my_location).unwrap();
        my_depth -= 1;
        total += 1;
    }

    while santa_depth > my_depth {
        santa_location = orbits.get(santa_location).unwrap();
        santa_depth -= 1;
        total += 1;
    }

    while my_location != santa_location {
        my_location = orbits.get(my_location).unwrap();
        santa_location = orbits.get(santa_location).unwrap();
        total += 2;
    }

    println!("Part 2: {}", total);
}
