
use std::collections::HashMap;
use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let input = read_lines("/Users/murilodeboni/advent_of_code/AoC2023/src/bin/day_two_input.txt");

    let ball_regex = Regex::new(r"(\d+) (blue|green|red)").unwrap();

    let mut bag = std::collections::HashMap::<String, i32>::new();

    let mut ans = 0;

    for i in input {
        bag.insert("red".to_string(), 0);
        bag.insert("green".to_string(), 0);
        bag.insert("blue".to_string(), 0);

        let pulls: Vec<_> = i.split(":").collect::<Vec<_>>()[1].split(";").collect();

        for pull in pulls {
            let balls: Vec<_> = pull.split(", ").collect();
            for ball in balls {
                let b = &ball_regex.captures(&ball).unwrap();
                let v = &b[1].parse::<i32>().unwrap();
                if bag.get(&b[2]).unwrap() < &v {   
                    bag.insert((&b[2]).to_string(), *v);
                }
            }
        }

        ans = ans + power(bag.clone())
    }
    println!("{}", ans)
}

fn power(bag: HashMap::<String, i32>) -> i32 {
    bag.get("red").unwrap() * bag.get("blue").unwrap() * bag.get("green").unwrap()
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}