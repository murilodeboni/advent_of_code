use std::time::Instant;

use aoc_utils::read_input;

const BASE: &str = env!("CARGO_MANIFEST_DIR");
const DAY: &str = "d02";

#[derive(Clone)]
struct Range {
    start: i64,
    end: i64
}

impl Range {
    fn print_range(&self) {
        println!("Range from {} to {}", self.start, self.end);
    }

    fn check_repeated(&self) -> Vec<i64> {
        let mut out = Vec::new();

        // Find first even digit length to consider
        let mut len = self.start.abs().to_string().len();
        if len % 2 == 1 { len += 1; }

        let max_len = self.end.abs().to_string().len();

        while len <= max_len {
            let half_len = len / 2;
            let pow10_half = 10_i64.pow(half_len as u32); // e.g. for len=4, 10^2=100
            let half_start = 10_i64.pow(half_len as u32 - 1); // e.g. for len=4, half_start=10
            let half_end = pow10_half - 1; // e.g. for len=4, half_end=99

            
            for half in half_start..=half_end {
                let rep = half * pow10_half + half; // e.g., 10*100 + 10 = 1010
                if rep > self.end { break; } // early stop within this length
                if rep >= self.start { out.push(rep); }
            }

            len += 2; // only even digit lengths
        }

        out
    }

    fn check_repeated_any(&self) -> Vec<i64> {
        let mut out = Vec::new();

        let max_len = self.end.abs().to_string().len();

        // Consider every possible total length up to max_len
        for len in 1..=max_len {
            for rep_len in 1..=len {
                // rep_len must divide len evenly; otherwise the pattern can't repeat to fill the length
                if len % rep_len != 0 {
                    continue;
                }

                // need at least two repeats to be considered "repeated"
                let repeats = len / rep_len;
                if repeats < 2 {
                    continue; 
                }

                // 10^rep_len
                let pow_chunk = 10_i64.pow(rep_len as u32);

                // Geometric multiplier: 1 + pow_chunk + pow_chunk^2 + ... + pow_chunk^(repeats-1)
                let pow_chunk_repeats = pow_chunk.pow(repeats as u32);

                let denom = pow_chunk - 1;
                if denom == 0 {
                    continue;
                }
                let numer = pow_chunk_repeats - 1;
                let geom = numer / denom; // safe because denom divides numer for geometric series

                // Chunk bounds to maintain digit width and fit inside range
                let chunk_min_base = if rep_len == 1 { 1 } else { 10_i64.pow(rep_len as u32 - 1) }; // avoid leading-zero chunks
                let chunk_max_base = pow_chunk - 1;

                let chunk_min = chunk_min_base.max((self.start + geom - 1) / geom); // ceil(start / geom)
                let chunk_max = chunk_max_base.min(self.end / geom);

                if chunk_min > chunk_max {
                    continue;
                }

                for chunk in chunk_min..=chunk_max {
                    // skip leading-zero chunks that would shrink total length
                    if chunk < chunk_min_base {
                        continue;
                    }
                    let val = chunk * geom;
                    if val >= self.start && val <= self.end {
                        out.push(val);
                    }
                }
            }
        }

        out.sort();
        out.dedup();
        out
    }
}

fn parse_input(input: &String) -> Vec<Range> {
    let ranges_str: Vec<&str> = input.split(',').collect();
    ranges_str.iter().map(|s| {
        let (start_str, end_str) = s.split_at(s.find('-').unwrap());
        let start = start_str.parse::<i64>().unwrap();
        let end = end_str[1..].parse::<i64>().unwrap();
        Range { start, end }
    }).collect()
}

fn main() {
    let start = Instant::now();
    let mut part1 = 0;
    let mut part2 = 0;

    let input = read_input(BASE, DAY, false);
    let range_input = parse_input(input.get(0).unwrap());

    range_input.iter().for_each(|r| {
        // r.print_range();
        let repeated = r.check_repeated();
        let repeated_any = r.check_repeated_any();
        part1 += repeated.iter().sum::<i64>();
        part2 += repeated_any.iter().sum::<i64>();
        // println!("Repeated numbers in range: {:?}", repeated);
        // println!("Repeated numbers (any size) in range: {:?}", repeated_any);
    });
    
    println!("{DAY} part1: {}", part1);
    println!("{DAY} part2: {}", part2);
    println!("Elapsed: {}µs", start.elapsed().as_micros());
}
