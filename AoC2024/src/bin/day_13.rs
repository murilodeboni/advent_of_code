mod utils;

use utils::input::read_lines;

use std::time::Instant;

#[derive(Debug)]
struct Behavior {
    a: (usize,usize),
    b: (usize,usize),
    price: (usize, usize)
}

fn main() {
    let start = Instant::now();
    let input = read_lines("./src/bin/inputs/day_13.txt");
    let mut numbers: Vec<Vec<usize>>  = Vec::new();
    let mut tokens: Vec<(isize,isize)> = Vec::new();

    for i in input {
        if i.is_empty() {
            continue;
        }

        let v = i
            .replace("Button ", "")
            .replace("A", "")
            .replace("B", "")
            .replace(": X+", "")
            .replace(" Y+", "")
            .replace("Prize: X=", "")
            .replace(" Y=", "");
        
            let n = v.split(",").map(|s| s.parse().expect("error parsing")).collect();
            numbers.push(n);

    }

    let mut part1 = 0;

    let mut i = 0;
    while i < numbers.len() {
        let behavior = Behavior {
            a: (numbers[i][0], numbers[i][1]),
            b: (numbers[i+1][0], numbers[i+1][1]),
            price: (numbers[i+2][0], numbers[i+2][1])
        };
        i=i+3;
        let (a,b) = solve_system(&behavior);
        println!("{:?} - (a={} b={}) - {} - {} {}",
         &behavior, 
         a, 
         b, 
         3*a+b, 
         (a*behavior.a.0 as isize + b*behavior.b.0 as isize) == behavior.price.0 as isize,
         (a*behavior.a.1 as isize + b*behavior.b.1 as isize) == behavior.price.1 as isize);

        if (a*behavior.a.0 as isize + b*behavior.b.0 as isize) == behavior.price.0 as isize&&
        (a*behavior.a.1 as isize + b*behavior.b.1 as isize) == behavior.price.1 as isize {
            tokens.push((a,b));
        }

    }

    // let valid_tokens: Vec<&(isize, isize)> = tokens.iter().filter(
    //     |(a,b)| a >= &0 && a<=&100 && b>=&0 && b<=&100
    // ).collect();

    // println!("{:?}", valid_tokens);

    let part1: isize = tokens.iter().map(|(a,b)| 3*a+b).sum();

    println!("{:?}", part1);
}

fn solve_system(behaviour: &Behavior) -> (isize,isize) {
    // A: ax+by=z --> x = (z-by)/a
    // B: cx+dy=w --> x = (w-dy)/c --> (z-by)/a = (w-dy)/c --> (zc-wa) = (cby-ady) --> y = (zc-wa)/(cb-ad)

    let a = behaviour.a.0 as isize;
    let b = behaviour.b.0 as isize;
    let c = behaviour.a.1 as isize;
    let d = behaviour.b.1 as isize;
    let z = behaviour.price.0 as isize;
    let w = behaviour.price.1 as isize;

    let y = (z*c-w*a)/(c*b-a*d);
    let x = (z-b*y)/a;

    // println!("x = {} y = {}", x,y);
    (x,y)
}