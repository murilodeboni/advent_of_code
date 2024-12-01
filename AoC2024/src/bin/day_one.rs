mod utils;

use utils::input::read_lines;

fn main() {
    let input = read_lines("/Users/murilodeboni/advent_of_code/AoC2024/src/bin/inputs/day_one.txt");

    let k_input = &input[..5];

    let mut left: Vec<usize> = Vec::new();
    let mut right: Vec<usize> = Vec::new();

    let l = input.len();

    for i in input {
        let parts: Vec<&str> = i.split("   ").collect();
        
        let p1: usize = parts[0].parse().expect("error");
        let p2: usize = parts[1].parse().expect("error");

        left.push(p1);
        right.push(p2);
    }

    left.sort();
    right.sort();

    let mut dists: usize = 0;

    for n in 0.. l {
        dists += left[n].abs_diff(right[n]);
    }

    println!("{}", dists);
}
