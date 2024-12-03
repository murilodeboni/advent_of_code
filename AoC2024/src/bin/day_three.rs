mod utils;

use utils::input::read_lines;

use regex::{Match, Regex};

fn main() {
    let input = read_lines("./src/bin/inputs/day_three.txt");

    println!("part 1 - {}", part1(&input));
    println!("part2 - {}", part2(&input));

}

fn part2(input: &Vec<String>) -> isize {
    let r = Regex::new(r"(mul\(\d+,\d+\))|(do)\(\)|(don't)\(\)").unwrap();

    let mut ans: isize = 0;
    let mut enabled = false;
    for i in input {
        let iter_ops = r.captures_iter(i.as_str());
        for op in iter_ops {
            for i in 1..op.len() {
                if let Some(matched) = op.get(i) {
                    if matched.as_str() == "do" {
                        enabled = false;
                    } else if matched.as_str() == "don't" || enabled == true {
                        enabled = true;
                    } else {
                        ans += get_ans_from_mul(matched);
                    }
                }
            }
        }
    }
    ans
}


fn part1(input: &Vec<String>) -> isize {
    let r = Regex::new(r"(mul\(\d+,\d+\))").unwrap();

    let mut ans: isize = 0;
    for i in input {
        let iter_ops = r.captures_iter(i.as_str());
        for op in iter_ops {
            for i in 1..op.len() {
                if let Some(matched) = op.get(i) {
                    ans += get_ans_from_mul(matched);
                }
            }
        }
    }
    ans
}

fn get_ans_from_mul(matched: Match<'_>) -> isize {
    let reduced_match = matched.as_str().replace("mul(", "").replace(")", "");
    let string_numbers: Vec<&str> = reduced_match.split(",").collect();
    // println!("{:?}", string_numbers);
    let numbers: Vec<isize> = string_numbers.iter().map(|l| l.parse::<isize>().expect("Failed to parse")).collect();
    numbers[0] * numbers[1]
}
