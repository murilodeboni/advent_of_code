mod utils;

use utils::input::read_lines;

use regex::Regex;

fn main() {
    let input = read_lines("./src/bin/inputs/day_three_test_2.txt");

    println!("{}", part1(&input))

}

fn part1(input: &Vec<String>) -> isize {
    let r = Regex::new(r"(mul\(\d+,\d+\))").unwrap();

    let mut ans: isize = 0;
    for i in input {
        let iter_ops = r.captures_iter(i.as_str());
        for op in iter_ops {
            for i in 1..op.len() {
                if let Some(matched) = op.get(i) {
                    let reduced_match = matched.as_str().replace("mul(", "").replace(")", "");
                    let string_numbers: Vec<&str> = reduced_match.split(",").collect();
                    // println!("{:?}", string_numbers);
                    let numbers: Vec<isize> = string_numbers.iter().map(|l| l.parse::<isize>().expect("Failed to parse")).collect();
                    ans += numbers[0] * numbers[1];
                }
            }
        }
    }
    ans
}
