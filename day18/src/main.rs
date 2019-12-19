use std::cmp::Reverse;
use std::collections::{BinaryHeap, BTreeSet, HashMap, HashSet, VecDeque};
use std::io::{stdin, BufRead};

struct Map {
    height: usize,
    width: usize,
    num_keys: usize,
    init_pos: (usize, usize),
    walkable: HashSet<(usize, usize)>,
    key_loc: HashMap<char, (usize, usize)>,
    loc_key: HashMap<(usize, usize), char>,
    door_loc: HashMap<char, (usize, usize)>,
    loc_door: HashMap<(usize, usize), char>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
struct State {
    num_steps: usize,
    prev: Option<char>,
    keys: BTreeSet<char>,
}

fn parse_map(input: &[Vec<char>]) -> Map {
    let mut res = Map {
        height: input.len(),
        width: input[0].len(),
        num_keys: 0,
        init_pos: (0, 0),
        walkable: HashSet::new(),
        key_loc: HashMap::new(),
        loc_key: HashMap::new(),
        door_loc: HashMap::new(),
        loc_door: HashMap::new(),
    };

    for (r, row) in input.iter().enumerate() {
        for (c, ch) in row.iter().enumerate() {
            match *ch {
                '#' => continue,
                '.' => {
                    res.walkable.insert((r, c));
                }
                '@' => {
                    res.walkable.insert((r, c));
                    res.init_pos = (r, c);
                }
                'a'..='z' => {
                    res.walkable.insert((r, c));
                    res.key_loc.insert(*ch, (r, c));
                    res.loc_key.insert((r, c), *ch);
                    res.num_keys += 1
                }
                'A'..='Z' => {
                    let key = ch.to_ascii_lowercase();

                    res.walkable.insert((r, c));
                    res.door_loc.insert(key, (r, c));
                    res.loc_door.insert((r, c), key);
                }
                _ => unreachable!(),
            }
        }
    }

    res
}

fn init_state() -> State {
    State {
        num_steps: 0,
        prev: None,
        keys: BTreeSet::new(),
    }
}

fn next_states(map: &Map, s: State) -> Vec<State> {
    let init_pos = match s.prev {
        None => map.init_pos,
        Some(ch) => *map.key_loc.get(&ch).unwrap(),
    };
    let init_steps = s.num_steps;

    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: VecDeque<((usize, usize), usize)> = VecDeque::new();

    seen.insert(init_pos);
    queue.push_back((init_pos, init_steps));

    let mut res: Vec<State> = Vec::new();

    while let Some(((r, c), steps)) = queue.pop_front() {
        // check if we got a key
        if let Some(nk) = map.loc_key.get(&(r, c)) {
            if !s.keys.contains(nk) {
                let mut ns = State {
                    num_steps: steps,
                    prev: Some(*nk),
                    keys: s.keys.clone(),
                };

                ns.keys.insert(*nk);

                res.push(ns);
                // we'll continue from this key in the outer search
                continue;
            }
        }

        let mut nps: Vec<(usize, usize)> = Vec::new();
        if r > 0 {
            nps.push((r - 1, c))
        }
        if c > 0 {
            nps.push((r, c - 1))
        }
        if r + 1 < map.height {
            nps.push((r + 1, c))
        }
        if c + 1 < map.width {
            nps.push((r, c + 1))
        }

        for np in nps {
            // check if we've been there before
            if seen.contains(&np) {
                continue;
            }

            // check for wall
            if !map.walkable.contains(&np) {
                continue;
            }

            // check for door that we can't unlock
            if let Some(ch) = map.loc_door.get(&np) {
                if !s.keys.contains(&ch) {
                    continue;
                }
            }

            // position is good
            seen.insert(np);
            queue.push_back((np, steps + 1));
        }
    }

    res
}

fn main() {
    let input: Vec<Vec<char>> = stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();

    // find all unreachable
    let map = parse_map(&input);
    let state = init_state();

    let mut seen: HashSet<(Option<char>, BTreeSet<char>)> = HashSet::new();
    let mut queue = BinaryHeap::new();

    seen.insert((state.prev, state.keys.clone()));
    queue.push(Reverse(state));

    while let Some(Reverse(s)) = queue.pop() {
        if s.keys.len() == map.num_keys {
            println!("{}", s.num_steps);
            return;
        }

        for ns in next_states(&map, s) {
            seen.insert((ns.prev, ns.keys.clone()));
            queue.push(Reverse(ns));
        }
    }
}
