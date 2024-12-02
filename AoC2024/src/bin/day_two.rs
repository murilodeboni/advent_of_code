mod utils;

use utils::input::read_lines;

fn main() {
    let input = read_lines("./src/bin/inputs/day_two.txt");

    let mut cnt_part_one = 0;
    let mut cnt_part_two = 0;

    for i in input {
        let levels_str: Vec<&str> = i.split(" ").collect();

        let levels: Vec<isize> = levels_str.iter().map(|l| l.parse::<isize>().expect("Failed to parse")).collect();
        
        cnt_part_one += if part1(&levels) {1} else {0};
        cnt_part_two += if part2(&levels) {1} else {0};
    }

    println!("{}", cnt_part_one);
    println!("{}", cnt_part_two);
}

fn part1(levels: &Vec<isize>) -> bool {
    let mut is_safe: bool = true;
    let is_increasing = levels[1] > levels[0];

    let s: usize = levels.len();

    let mut i = 1;
    while is_safe && i <= s-1 {

        let l0 = levels[i-1];
        let l1 = levels[i];
        if is_increasing {
            is_safe = {
                l1 > l0 && l1 - l0 <=3
            }
        } else {
            is_safe = l1 < l0 && l0 - l1 <=3
        }
        i += 1;
    }
    
    // println!("{:?} {} {}", levels, i, is_safe);
    
    is_safe
}

fn part2(levels: &Vec<isize>) -> bool {
    let s: usize = levels.len();

    for i in 0..s {
        let left_part= levels[..i].to_vec();
        let right_part = levels[i + 1..].to_vec();
        
        if part1(&[left_part, right_part].concat()) {
            return true;
        }
    }
    return false;
}