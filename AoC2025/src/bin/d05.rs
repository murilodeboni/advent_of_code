use std::time::Instant;
use aoc_utils::read_input;

const BASE: &str = env!("CARGO_MANIFEST_DIR");
const DAY: &str = "d05";

struct Range {
    start: u128,
    end: u128,
}

struct Fresh {
    f: Vec<u128>
}

fn parse_input(input: Vec<String>) -> (Vec<Range>, Vec<Fresh>) {
    let mut ranges: Vec<Range> = Vec::new();
    let mut fresh: Vec<Fresh> = Vec::new();
    let mut parsing_fresh = false;

    for line in input {
        if line.trim().is_empty() {
            parsing_fresh = true;
            continue;
        }

        if !parsing_fresh {
            let (start, end) = line
                .split_once('-')
                .expect("range line must contain '-'");

            ranges.push(Range {
                start: start.parse().expect("start should be a number"),
                end: end.parse().expect("end should be a number"),
            });
        } else {
            fresh.push(Fresh {
                f: vec![line.parse().expect("fresh value should be a number")],
            });
        }
    }

    (ranges, fresh)
}

fn combined_ranges(mut ranges: Vec<Range>) -> Vec<Range> {
    ranges.sort_by_key(|r| r.start);

    let mut combined: Vec<Range> = Vec::new();

    for r in ranges {
        if let Some(last) = combined.last_mut() {
            if r.start <= last.end + 1 {
                last.end = last.end.max(r.end);
                continue;
            }
        }
        combined.push(Range { start: r.start, end: r.end });
    }

    combined
}

fn main() {
    let start = Instant::now();

    let mut part1: u128 = 0;
    let mut part2: u128 = 0;

    let input = read_input(BASE, DAY, false);

    let (ranges, fresh) = parse_input(input);

    for f in &fresh {
        for r in &ranges {
            if f.f[0] >= r.start && f.f[0] <= r.end {
                part1 += 1;
                break;
            }
        }
    }

    let combined = combined_ranges(ranges);
    for r in &combined {
        part2 += r.end - r.start + 1;
    }

    println!("{DAY} part1: {}", part1);
    println!("{DAY} part2: {}", part2);
    println!("Elapsed: {}us", start.elapsed().as_micros());
}
