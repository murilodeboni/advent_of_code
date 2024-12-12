mod utils;

use utils::input::read_lines;

use std::{collections::HashMap, time::Instant};

fn main() {
    let start = Instant::now();
    let input = read_lines("./src/bin/inputs/day_11_test.txt");
    let rocks: Vec<usize> = input[0]
        .split(" ")
        .map(|l| l
            .parse()
            .expect("error")
        ).collect();

    let mut d: HashMap<usize, usize> = HashMap::new();
    for rock in rocks {
        d.insert(rock, 1);
    }


    let mut part1: usize;
    let part2: usize;
    
    for i in 0..6 {
        println!("{}", i);
        blink(&mut d);
        if i == 24 {
            part1 = d.values().sum();
            println!("part 1: {} took {}ms", part1, start.elapsed().as_millis());
        }
        print_dict(d.clone());
    }

    part2 = d.values().sum();
    println!("part 2: {}", part2);


}

fn blink(d: &mut HashMap<usize, usize>) {
    let rocks: Vec<(usize, usize)> = d.iter()
    .filter(|&(_, &v)| v > 0)
    .map(|(&k, &v)| (k, v))
    .collect();

    for (rock, v) in rocks.clone() {
        if let Some(value) = d.get_mut(&rock) {
            if *value > 0 {
                *value = 0;
            }
        }
        
        if rock == 0 {
            *d.entry(1).or_insert(0) += v;
        } else if has_even_digits(&rock) {
            for r in split(&rock) {
                *d.entry(r).or_insert(0) += v;
            }
        } else {
            *d.entry(rock*2024).or_insert(0) += v;
        }
    }
}

fn has_even_digits(u: &usize) -> bool {
    let num_digits = u.to_string().len();
    num_digits % 2 == 0
}

fn split(n: &usize) -> Vec<usize> {
    let num_str = n.to_string();
    let len = num_str.len();
    let mid = len / 2;

    let first_half = &num_str[0..mid];
    let second_half = &num_str[mid..];

    vec![
        first_half.parse::<usize>().unwrap(),
        second_half.parse::<usize>().unwrap(),
    ]
}

fn print_dict(d: HashMap<usize, usize>) {
    let mut vector: Vec<usize> = Vec::new();
    for (k,v) in d {
        for i in 0..v {
        if v > 0 {
            vector.push(k);
        }
    }
    }
    vector.sort();
    println!("{:?}", vector);
}