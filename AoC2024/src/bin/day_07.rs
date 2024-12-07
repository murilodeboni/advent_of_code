mod utils;

use std::time::Instant;

use utils::input::read_lines;

fn main() {
    let input = read_lines("./src/bin/inputs/day_07.txt");
    let mut part1_ans:usize = 0;
    let mut part2_ans:usize = 0;

    let start = Instant::now();

    for i in input {
        let mut right: Vec<usize> = Vec::new();

        let parts: Vec<&str> = i.split(": ").collect();
        let left: usize = parts[0].parse().expect("error");

        let right_side = parts[1].split(" ");
        for (_, value) in right_side.enumerate() {
            right.push(value.parse().expect("error"));
        }
        
        part1_ans += part1(left, right.clone(), Vec::new());
        part2_ans += part2(left, right, Vec::new());

    }

    let final_time = start.elapsed();

    println!("{}", part1_ans);
    println!("{}", part2_ans);
    println!("{}", final_time.as_millis())
}

fn part1(left: usize, mut right: Vec<usize>, possbilities: Vec<usize>) -> usize {
    while !right.is_empty() {
        let v = right[0];
        if possbilities.is_empty() {
            return part1(left, right.split_off(1) , vec![v]);
        } else {
            let mut new_possibilities: Vec<usize> = Vec::new();
            for p in &possbilities {

                let new_v_mult: usize = p*v;
                let new_v_sum: usize = p+v;

                if new_v_mult <= left {
                    new_possibilities.push(new_v_mult);
                }

                if new_v_sum <= left {
                    new_possibilities.push(new_v_sum);
                }

            }
            if new_possibilities.is_empty() {
                return 0;
            }

            return part1(left, right.split_off(1) , new_possibilities);
        }
    }
    return if possbilities.contains(&left) {left} else {0};
}

fn part2(left: usize, mut right: Vec<usize>, possbilities: Vec<usize>) -> usize {
    while !right.is_empty() {
        let v = right[0];
        if possbilities.is_empty() {
            return part2(left, right.split_off(1) , vec![v]);
        } else {
            let mut new_possibilities: Vec<usize> = Vec::new();
            for p in &possbilities {

                let new_v_mult: usize = p*v;
                let new_v_sum: usize = p+v;
                let new_v_concat: usize = concat(*p, v);

                if new_v_mult <= left {
                    new_possibilities.push(new_v_mult);
                }

                if new_v_sum <= left {
                    new_possibilities.push(new_v_sum);
                }

                if new_v_concat <= left {
                    new_possibilities.push(new_v_concat);
                }
            }
            if new_possibilities.is_empty() {
                return 0;
            }

            return part2(left, right.split_off(1) , new_possibilities);
        }
    }
    return if possbilities.contains(&left) {left} else {0};
}

fn concat(a: usize, b: usize) -> usize {
    a * 10usize.pow(b.ilog10() + 1) + b
}