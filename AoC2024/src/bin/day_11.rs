mod utils;

use utils::input::read_lines;

use std::time::Instant;

fn main() {
    let start = Instant::now();
    let input = read_lines("./src/bin/inputs/day_11.txt");
    let mut rocks: Vec<usize> = input[0]
        .split(" ")
        .map(|l| l
            .parse()
            .expect("error")
        ).collect();

    let mut part1: usize = 0; 
    
    for i in 0..75 {
        println!("{}", i);
        rocks = blink(rocks);
        if i == 24 {
            part1 = rocks.len();
            println!("part 1: {}", part1);
        }
    }
    println!("part 2: {}", rocks.len());


}

fn blink(rocks:Vec<usize>) -> Vec<usize> {
    let mut new_rocks: Vec<usize> = Vec::new();
    for rock in rocks {
        if rock == 0 {
            new_rocks.push(1);
        } else if has_even_digits(&rock) {
            new_rocks.append(&mut split(&rock))
        } else {
            new_rocks.push(rock*2024);
        }
    }
    new_rocks
}

fn has_even_digits(u: &usize) -> bool {
    let num_digits = u.to_string().len(); // Convert to string and get length
    num_digits % 2 == 0 // Check if the length is even
}

fn split(n: &usize) -> Vec<usize> {
    let num_str = n.to_string(); // Convert the number to a string
    let len = num_str.len();
    let mid = len / 2; // Calculate the middle point

    let first_half = &num_str[0..mid];
    let second_half = &num_str[mid..];

    vec![
        first_half.parse::<usize>().unwrap(),
        second_half.parse::<usize>().unwrap(),
    ]
}