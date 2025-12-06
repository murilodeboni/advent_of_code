use std::time::Instant;

use aoc_utils::read_input;

const BASE: &str = env!("CARGO_MANIFEST_DIR");
const DAY: &str = "d03";

#[derive(Clone)]
struct Bank {
    battery: Vec<u32>,
    p1: u128,
    p2: u128,
}

impl Bank {
    fn print (&self) {
        for b in &self.battery {
            print!("{b} ");
        }
        println!();
    }

    fn max_capacity(&mut self, digits: usize, is_p2: bool) {
        let digits = digits.min(self.battery.len());
        let mut drop = self.battery.len().saturating_sub(digits);
        let mut stack: Vec<u32> = Vec::with_capacity(self.battery.len());

        for &d in self.battery.iter() {
            while drop > 0 && stack.last().is_some_and(|&last| last < d) {
                stack.pop();
                drop -= 1;
            }
            stack.push(d);
        }
        stack.truncate(digits);

        let value = stack
            .iter()
            .take(digits)
            .fold(0u128, |acc: u128, &digit| acc * 10 + digit as u128);

        if is_p2 {self.p2 += value} else { self.p1 += value };
    }

}

fn parse_input(input: &String) -> Vec<Bank> {
    input.lines().map(|line| {
        let battery: Vec<u32> = line
            .chars()
            .map(|c| c.to_digit(10).expect("only digits"))
            .collect();
        Bank { battery, p1: 0 , p2: 0 }
    }).collect()
}

fn main() {
    let start = Instant::now();
    let mut part1 = 0;
    let mut part2 = 0;

    let input = read_input(BASE, DAY, false);
    let mut range_input: Vec<Bank> = input.iter().map(|i| parse_input(i)).flatten().collect();
    range_input.iter_mut().for_each(|f| {
        f.max_capacity(2, false);
        f.max_capacity(12, true);
    });

    range_input.iter().for_each(|r| {
        part1 += r.p1;
        part2 += r.p2;
    });

    println!("{DAY} part1: {}", part1);
    println!("{DAY} part2: {}", part2);
    println!("Elapsed: {}µs", start.elapsed().as_micros());
}
