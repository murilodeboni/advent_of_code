use std::time::Instant;

use aoc_utils::read_input;

const BASE: &str = env!("CARGO_MANIFEST_DIR");
const DAY: &str = "d02";

#[derive(Clone)]
struct Range {
    start: i64,
    end: i64,
    p1: Vec<i64>,
    p2: Vec<i64>,
}

impl Range {
    fn print_range(&self) {
        println!("Range from {} to {}", self.start, self.end);
    }

    fn check_repeated(&mut self) {
        for i in self.start..=self.end {
            let s = i.to_string();
            let len = s.len();
            
            for rep_len in (1..=(len / 2)).rev() {
                if len % rep_len != 0 {
                    continue;
                }
                let is_p1 = rep_len == len / 2 && len % 2 == 0;
                
                let chunk = &s[0..rep_len];
                let mut repeated = String::new();
                let repeats = len / rep_len;
                for _ in 0..repeats {
                    repeated.push_str(chunk);
                }

                if repeated == s {
                    // println!("Found repeated number: {} rep_len {} ren {}", i, rep_len, len);
                    self.p2.push(i);
                    if is_p1 { self.p1.push(i) };
                    break;
                }
            }
        }
    }
}

fn parse_input(input: &String) -> Vec<Range> {
    let ranges_str: Vec<&str> = input.split(',').collect();
    ranges_str.iter().map(|s| {
        let (start_str, end_str) = s.split_at(s.find('-').unwrap());
        let start = start_str.parse::<i64>().unwrap();
        let end = end_str[1..].parse::<i64>().unwrap();
        Range { start, end, p1: Vec::new(), p2: Vec::new() }
    }).collect()
}

fn main() {
    let start = Instant::now();
    let mut part1 = 0;
    let mut part2 = 0;

    let input = read_input(BASE, DAY, false);
    let mut range_input = parse_input(input.get(0).unwrap());

    range_input.iter_mut().for_each(|r| {
        // r.print_range();
        r.check_repeated();
        part1 += r.p1.iter().sum::<i64>();
        part2 += r.p2.iter().sum::<i64>();
        // println!("Repeated numbers in range: {:?}", r.p1);
        // println!("Repeated numbers (any size) in range: {:?}", r.p2);
    });
    
    println!("{DAY} part1: {}", part1);
    println!("{DAY} part2: {}", part2);
    println!("Elapsed: {}µs", start.elapsed().as_micros());
}
