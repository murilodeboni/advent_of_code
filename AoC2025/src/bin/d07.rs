use std::collections::{HashSet, VecDeque};
use std::time::Instant;
use aoc_utils::read_input;

const BASE: &str = env!("CARGO_MANIFEST_DIR");
const DAY: &str = "d07";


struct TachyonManifold {
    data: Vec<Vec<char>>,
    beams: Vec<(usize, usize)>,
    splits: usize,
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
        beams: vec![b],
        splits: 0,
    }
}

fn main() {
    let start = Instant::now();

    let mut part2: u128 = 0;

    let input = read_input(BASE, DAY, false);

    let mut manifold = parse_input(input);
    manifold.follow_beams();

    println!("{DAY} part1: {}", manifold.splits);
    println!("{DAY} part2: {}", part2);
    println!("Elapsed: {}us", start.elapsed().as_micros());
}
