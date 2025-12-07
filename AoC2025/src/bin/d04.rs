use std::collections::VecDeque;
use std::time::Instant;
use aoc_utils::read_input;

const BASE: &str = env!("CARGO_MANIFEST_DIR");
const DAY: &str = "d04";

#[derive(Clone)]
struct Wharehouse {
    matrix: Vec<Vec<bool>>,
    neighbor_counts: Vec<Vec<u8>>,
    p1: u128,
    p2: u128
}

impl Wharehouse {
    fn print(&self) {
        for row in &self.matrix {
            println!("{:?}", row);
        }
    }

    /// Precompute neighbor counts for every cell.
    fn neighbor_counts(&mut self) {
        let height = self.matrix.len();
        let width = self.matrix.first().map(|r| r.len()).unwrap_or(0);

        let directions: [(i32, i32); 8] = [
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1),           (0, 1),
            (1, -1),  (1, 0),  (1, 1),
        ];

        let mut counts = vec![vec![0u8; width]; height];

        for y in 0..height {
            for x in 0..width {
                if !self.matrix[y][x] {
                    continue;
                }

                let mut c = 0u8;
                for (dy, dx) in directions.iter() {
                    let ny = y as i32 + dy;
                    let nx = x as i32 + dx;

                    if nx >= 0
                        && nx < width as i32
                        && ny >= 0
                        && ny < height as i32
                        && self.matrix[ny as usize][nx as usize]
                    {
                        c += 1;
                    }
                }
                counts[y][x] = c;
            }
        }

        self.neighbor_counts = counts;
    }

    /// Part 1: count how many cells start with fewer than `max` neighbors.
    fn how_many_can_access(&mut self, max: u8) {
        let mut total: u128 = 0;

        for y in 0..self.matrix.len() {
            for x in 0..self.matrix[0].len() {
                if self.matrix[y][x] && self.neighbor_counts[y][x] < max {
                    total += 1;
                }
            }
        }

        self.p1 = total;
    }

    /// Part 2: iteratively peel cells with neighbour count < max using a queue.
    fn how_many_can_access_iter(&mut self, max: u8) {
        let height = self.matrix.len();
        let width = self.matrix[0].len();

        let mut queue = VecDeque::new();

        for y in 0..height {
            for x in 0..width {
                if self.matrix[y][x] && self.neighbor_counts[y][x] < max {
                    queue.push_back((y, x));
                }
            }
        }

        let directions: [(i32, i32); 8] = [
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1),           (0, 1),
            (1, -1),  (1, 0),  (1, 1),
        ];

        while let Some((y, x)) = queue.pop_front() {
            if !self.matrix[y][x] {
                continue; // already removed
            }

            self.matrix[y][x] = false;
            self.p2 += 1;

            for (dy, dx) in directions.iter() {
                let ny = y as i32 + dy;
                let nx = x as i32 + dx;

                if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                    let nyu = ny as usize;
                    let nxu = nx as usize;

                    if self.matrix[nyu][nxu] {
                        // Neighbor loses one adjacent cell; push if it now falls below threshold.
                        if self.neighbor_counts[nyu][nxu] > 0 {
                            self.neighbor_counts[nyu][nxu] -= 1;
                        }
                        if self.neighbor_counts[nyu][nxu] < max {
                            queue.push_back((nyu, nxu));
                        }
                    }
                }
            }
        }
    }
}

fn parse_input(input: Vec<String>) -> Wharehouse {
    let mut matrix: Vec<Vec<bool>> = Vec::new();

    for line in input {
        let row: Vec<bool> = line.chars().map(|c| c == '@').collect();
        matrix.push(row);
    }

    Wharehouse {
        matrix,
        neighbor_counts: vec![],
        p1: 0,
        p2: 0,
    }
}

fn main() {
    let start = Instant::now();

    let input = read_input(BASE, DAY, false);
    let mut whs1: Wharehouse = parse_input(input);
    whs1.neighbor_counts();

    let mut whs2 = whs1.clone();

    whs1.how_many_can_access(4);
    whs2.how_many_can_access_iter(4);

    println!("{DAY} part1: {}", whs1.p1);
    println!("{DAY} part2: {}", whs2.p2);
    println!("Elapsed: {}ms", start.elapsed().as_millis());
}
