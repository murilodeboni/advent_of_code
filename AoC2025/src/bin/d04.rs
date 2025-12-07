use std::time::Instant;

use aoc_utils::read_input;

const BASE: &str = env!("CARGO_MANIFEST_DIR");
const DAY: &str = "d04";

#[derive(Clone)]
struct Wharehouse {
    matrix: Vec<Vec<bool>>,
    p1: u128,
    p2: u128
}

impl Wharehouse {
    fn print (&self) {
        for row in &self.matrix {
            println!("{:?}", row);
        }
    }

    fn count_adjacent(&self, y: usize, x: usize) -> u128 {
        let directions: [(i32, i32); 8] = [
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1),           (0, 1),
            (1, -1),  (1, 0),  (1, 1),
        ];

        let mut count: u128 = 0;

        for (dx, dy) in directions.iter() {
            let new_x = x as i32 + dx;
            let new_y = y as i32 + dy;

            if new_x >= 0 && new_x < self.matrix[0].len() as i32 &&
               new_y >= 0 && new_y < self.matrix.len() as i32 {
                if self.matrix[new_x as usize][new_y as usize] {
                    count += 1;
                }
            }
        }

        count
    }

    fn how_many_can_access(&mut self, max: u128) {
        let mut count: u128 = 0;
        for y in 0..self.matrix.len() {
            for x in 0..self.matrix.get(0).unwrap().len() {
                count += if  self.matrix[y][x] && self.count_adjacent(x, y) < max { 
                    // println!("({}, {}) can access", x, y);
                    1 
                } else { 0 }
            }
        }
        self.p1 = count;
    }

    fn how_many_can_access_rec(&mut self, max: u128) {
        for y in 0..self.matrix.len() {
            for x in 0..self.matrix.get(0).unwrap().len() {
                if self.matrix[y][x] && self.count_adjacent(x, y) < max { 
                    // println!("({}, {}) can access", x, y);
                    self.matrix[y][x] = false;
                    self.p2 += 1;
                    self.how_many_can_access_rec(max);
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
        p1: 0,
        p2: 0,
    }
}

fn main() {
    let start = Instant::now();

    let input = read_input(BASE, DAY, false);
    let mut whs1: Wharehouse = parse_input(input);
    let mut whs2 = whs1.clone();

    whs1.how_many_can_access(4);
    whs2.how_many_can_access_rec(4);

    println!("{DAY} part1: {}", whs1.p1);
    println!("{DAY} part2: {}", whs2.p2);
    println!("Elapsed: {}µs", start.elapsed().as_micros());
}
