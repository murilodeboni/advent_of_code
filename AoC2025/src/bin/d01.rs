use std::time::Instant;

use aoc_utils::read_input;

const BASE: &str = env!("CARGO_MANIFEST_DIR");
const DAY: &str = "d01";

struct DirectionInput {
    direction: char,
    value: i64,
}

struct Dial {
    position: i64,
    part1: i64,
    part2: i64,
} 

impl Dial {
    fn new() -> Self {
        Dial { position: 50 , part1: 0, part2: 0 }
    }

    fn turn(&mut self, direction: char, value: i64) {
        let rest = value % 100;
        self.part2 += value / 100;

        let delta = match direction {
            'L' => -rest,
            'R' => rest,
            _ => panic!("Invalid direction"),
        };

        let wrapped = match direction {
            'L' => self.position != 0 && rest > self.position,
            'R' => self.position + rest >= 100,
            _ => unreachable!(),
        };

        self.position = (self.position + delta).rem_euclid(100);

        if wrapped && self.position != 0 {
            self.part2 += 1;
        }

        if self.position == 0 && rest > 0 {
            self.part1 += 1;
            self.part2 += 1;
        }
    }

    fn print_position(&self) {
        println!("Dial position: {}, cnt {}", self.position, self.part2);
    }
}

fn parse_input(input: &[String]) -> Vec<DirectionInput> {
    input.iter().map(|s| {
        let (dir, val) = s.split_at(1);
        DirectionInput {
            direction: dir.chars().next().unwrap(),
            value: val.parse().unwrap(),
        }
    }).collect()
}

fn main() {
    let input = read_input(BASE, DAY, false);

    let dir_input: Vec<DirectionInput> = parse_input(&input);
    
    let start = Instant::now();
    let mut dial = Dial::new();
    dir_input.iter().for_each(|di| {
        dial.turn(di.direction, di.value);
    });
    let elapsed = start.elapsed();

    println!("{DAY} part1: {} ({}µs)", dial.part1, elapsed.as_micros());
    println!("{DAY} part2: {} ({}µs)", dial.part2, elapsed.as_micros());
}
