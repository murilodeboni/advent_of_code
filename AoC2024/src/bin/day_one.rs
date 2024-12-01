mod utils;

use std::collections::HashMap;

use utils::input::read_lines;

fn main() {
    let input = read_lines("./src/bin/inputs/day_one.txt");

    let mut left: Vec<usize> = Vec::new();
    let mut right: Vec<usize> = Vec::new();

    for i in input {
        let parts: Vec<&str> = i.split("   ").collect();
        
        let p1: usize = parts[0].parse().expect("error");
        let p2: usize = parts[1].parse().expect("error");

        left.push(p1);
        right.push(p2);
    }

    part1(&left, &right);

    part2(&left, &right);
}

fn part1(l: &Vec<usize>, r: &Vec<usize>) {
    let mut left = l.clone();
    let mut right = r.clone();

    left.sort();
    right.sort();

    let mut dists: usize = 0;

    for n in 0.. left.len() {
        dists += left[n].abs_diff(right[n]);
    }

    println!("{}", dists);
}

fn part2(l: &Vec<usize>, r: &Vec<usize>) {
    let mut map_r: HashMap<usize, usize> = HashMap::new();

    for n in r {
        *map_r.entry(*n).or_insert(0) += 1;
    }

    let mut similarity: usize = 0;

    for n in l {
        if let Some(rv) = map_r.get(n) {
            similarity += n*rv;
        }
    }

    println!("{}", similarity);

}
