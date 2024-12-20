mod utils;

use utils::input::read_lines;

use std::time::Instant;


fn main() {
    let start_time = Instant::now();
    let input = read_lines("./src/bin/inputs/day_19_1.txt");
    let patterns: Vec<&str> = input[0].split(", ").collect();
    let input2= read_lines("./src/bin/inputs/day_19_2.txt");
    let towels: Vec<&str> = input2.iter().map(|s| s.as_str()).collect();
    let mut part1: usize = 0;

    for towel in towels {
        println!("trying to form '{}' ", towel);
        if is_possible(towel, &patterns) {part1 += 1}
    }
    println!("part 1 - {} took {}ms", part1, start_time.elapsed().as_millis());
}

fn is_possible(target: &str, elements: &Vec<&str>) -> bool {
    // println!("remaining {}", target);
    if target.is_empty() {
        return true;
    }
    for &element in elements {
        if target.starts_with(element) {
            let remaining = &target[element.len()..];
            if is_possible(remaining, elements) {
                return true;
            }
        }
    }
    false
}