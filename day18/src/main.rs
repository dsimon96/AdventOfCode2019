use std::cmp::Reverse;
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
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

#[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
enum Node {
    Start,
    Door(char),
    Key(char),
}

#[derive(Debug)]
struct Graph {
    node_loc: HashMap<Node, (usize, usize)>,
    loc_node: HashMap<(usize, usize), Node>,
    adj: HashMap<Node, Vec<(Node, usize)>>,
}

fn do_dijkstra(
    map: &Map,
    s: &Node,
    pos: &(usize, usize),
    loc_node: &HashMap<(usize, usize), Node>,
    adj: &mut HashMap<Node, Vec<(Node, usize)>>,
) {
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: VecDeque<((usize, usize), usize)> = VecDeque::new();

    seen.insert(*pos);
    queue.push_back((*pos, 0));

    while let Some((p, steps)) = queue.pop_front() {
        if steps > 0 {
            if let Some(t) = loc_node.get(&p) {
                adj.entry(s.clone()).or_default().push((t.clone(), steps));
                continue;
            }
        }

        // add adjacent positions
        let (r, c) = p;
        let mut pos: Vec<(usize, usize)> = Vec::new();
        if r > 0 {
            pos.push((r - 1, c));
        }
        if c > 0 {
            pos.push((r, c - 1));
        }
        if r + 1 < map.height {
            pos.push((r + 1, c));
        }
        if c + 1 < map.width {
            pos.push((r, c + 1));
        }

        for np in &pos {
            if seen.contains(np) || !map.walkable.contains(np) {
                continue;
            }

            seen.insert(*np);
            queue.push_back((*np, steps + 1));
        }
    }
}

fn path_map(map: &Map) -> Graph {
    let mut g = Graph {
        node_loc: HashMap::new(),
        loc_node: HashMap::new(),
        adj: HashMap::new(),
    };

    g.node_loc.insert(Node::Start, map.init_pos);
    g.loc_node.insert(map.init_pos, Node::Start);

    for (&ch, &pos) in &map.key_loc {
        g.node_loc.insert(Node::Key(ch), pos);
        g.loc_node.insert(pos, Node::Key(ch));
    }

    for (&ch, &pos) in &map.door_loc {
        g.node_loc.insert(Node::Door(ch), pos);
        g.loc_node.insert(pos, Node::Door(ch));
    }

    for (v, pos) in &g.node_loc {
        do_dijkstra(&map, v, pos, &g.loc_node, &mut g.adj);
    }

    g
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct State {
    pos: Node,
    keys: BTreeSet<char>,
}

fn main() {
    let input: Vec<Vec<char>> = stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();

    let map = parse_map(&input);
    let routes = path_map(&map);

    let mut handled: HashSet<State> = HashSet::new();
    let mut seen: HashMap<State, usize> = HashMap::new();
    let mut queue: BinaryHeap<Reverse<(usize, State)>> = BinaryHeap::new();

    let init_state = State {
        pos: Node::Start,
        keys: BTreeSet::new(),
    };

    seen.insert(init_state.clone(), 0);
    queue.push(Reverse((0, init_state)));
    while let Some(Reverse((steps, s))) = queue.pop() {
        if !handled.insert(s.clone()) {
            continue;
        }

        if s.keys.len() == map.num_keys {
            println!("{}", steps);
            return;
        }

        for (npos, n) in routes.adj.get(&s.pos).unwrap() {
            let mut ns = State {
                pos: npos.clone(),
                keys: s.keys.clone(),
            };

            if let Node::Door(ch) = npos {
                if !s.keys.contains(ch) {
                    continue;
                }
            }

            if let Node::Key(ch) = npos {
                ns.keys.insert(*ch);
            }

            let new_dist = steps + n;
            let dist = seen.entry(ns.clone()).or_insert(new_dist);

            if new_dist > *dist {
                continue;
            }

            *dist = new_dist;

            queue.push(Reverse((new_dist, ns)));
        }
    }
}
