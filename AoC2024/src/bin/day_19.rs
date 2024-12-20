mod utils;

use utils::input::read_lines;

use std::{collections::HashMap, time::Instant};


fn main() {
    let start_time = Instant::now();
    let input = read_lines("./src/bin/inputs/day_19_1.txt");
    let patterns: Vec<&str> = input[0].split(", ").collect();
    let input2= read_lines("./src/bin/inputs/day_19_2.txt");
    let towels: Vec<&str> = input2.iter().map(|s| s.as_str()).collect();
    let mut memo = &mut HashMap::new();
    let mut part1: usize = 0;
    let mut part2: usize = 0;

    for towel in towels {
        println!("trying to form '{}' ", towel);
        let ans = is_possible(towel, &patterns, &mut memo);
        if ans > 0 {
            part1 += 1;
            part2 += ans;
        }
    }

    println!("part 1 - {}, part 2 - {} | took {}ms", part1, part2, start_time.elapsed().as_millis());
}

fn is_possible(target: &str, elements: &Vec<&str>, memo: &mut HashMap<String, usize>) -> usize {
    if let Some(&result) = memo.get(target) {
        return result;
    }

    if target.is_empty() {
        return 1;
    }

    let mut total_ways = 0;

    for &element in elements {
        if target.starts_with(element) {
            let remaining = &target[element.len()..];
            total_ways += is_possible(remaining, elements, memo);
        }
    }
    memo.insert(target.to_string(), total_ways);
    total_ways
}