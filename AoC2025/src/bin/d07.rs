use std::collections::{HashMap, HashSet, VecDeque};
use std::time::Instant;
use aoc_utils::read_input;

const BASE: &str = env!("CARGO_MANIFEST_DIR");
const DAY: &str = "d07";

#[derive(Clone)]
struct TachyonManifold {
    data: Vec<Vec<char>>,
    start: (usize, usize),
    beams: Vec<(usize, usize)>,
    map: HashMap<(usize, usize), usize>,
    splits: usize,
    splits2: usize,
} 

impl TachyonManifold {
    fn is_split(&self, position: &(usize, usize)) -> bool {
        let &(x, y) = position;
        if self.data[y][x] == '^' {
            return true;
        }
        false
    }

    fn follow_beams(&mut self) {
        self.beams.push(self.start);
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        let mut in_queue: HashSet<(usize, usize)> = HashSet::new();

        for beam in self.beams.drain(..) {
            if in_queue.insert(beam) {
                queue.push_back(beam);
            }
        }

        let height = self.data.len();
        let length = self.data[0].len();

        while let Some((x, y)) = queue.pop_front() {
            in_queue.remove(&(x, y));

            if y + 1 >= height {
                continue;
            }

            let next_y = y + 1;

            if self.is_split(&(x, y)) {
                self.splits += 1;

                if x > 0 && in_queue.insert((x - 1, next_y)) {
                    queue.push_back((x - 1, next_y));
                }

                if x + 1 < length && in_queue.insert((x + 1, next_y)) {
                    queue.push_back((x + 1, next_y));
                }
            } else if x < length && in_queue.insert((x, next_y)) {
                queue.push_back((x, next_y));
            }
        }

        self.beams = queue.into();
    }

    /// Memoized count of splits/universes reachable from a position.
    fn universes_from(&mut self, position: (usize, usize)) -> usize {
        if let Some(&cached) = self.map.get(&position) {
            return cached;
        }

        let (x, y) = position;
        let height = self.data.len();

        if y + 1 >= height {
            self.map.insert(position, 0);
            return 0;
        }

        let next_y = y + 1;
        let width = self.data[next_y].len();

        let universes = if self.is_split(&(x, y)) {
            let left = if x > 0 { self.universes_from((x - 1, next_y)) } else { 0 };
            let right = if x + 1 < width { self.universes_from((x + 1, next_y)) } else { 0 };
            1 + left + right // count this split plus branches
        } else if x < width {
            self.universes_from((x, next_y))
        } else {
            0
        };

        self.map.insert(position, universes);
        universes
    }

    fn multiworld_travel(&mut self) {
        // Use memoized universes_from to avoid traversing every branch repeatedly.
        self.splits2 += self.universes_from(self.start);
    }
}

fn parse_input(input: Vec<String>) -> TachyonManifold {
    let t: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();
    let mut b: (usize, usize) = (0,0);
    
    for y in 0..t.len() {
        for x in 0..t[y].len() {
            if t[y][x] == 'S' {
                b = (x, y);
                break;
            }
        }
    };

    TachyonManifold {
        data: t,
        start: b,
        beams: Vec::new(),
        splits: 0,
        splits2: 1,
        map: HashMap::new(),
    }
}

fn main() {
    let start = Instant::now();

    let input = read_input(BASE, DAY, false);

    let mut manifold = parse_input(input);

    manifold.follow_beams();
    manifold.multiworld_travel();

    println!("{DAY} part1: {}", manifold.splits);
    println!("{DAY} part2: {}", manifold.splits2);
    println!("Elapsed: {}us", start.elapsed().as_micros());
}
