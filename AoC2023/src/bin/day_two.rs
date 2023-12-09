use std::collections::HashMap;
use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let input = read_lines("/Users/murilodeboni/advent_of_code/AoC2023/src/bin/day_two_input.txt");

    let game_regex = Regex::new(r"^Game (\d+).*$").unwrap();
    let ball_regex = Regex::new(r"(\d+) (blue|green|red)").unwrap();

    let mut possible = true;

    let bag = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14)
    ]);

    let mut ans = 0;

    for i in input {
        possible = true;
        let game_number = &game_regex.captures(&i).unwrap()[1].parse::<i32>().unwrap();

        let pulls: Vec<_> = i.split(":").collect::<Vec<_>>()[1].split(";").collect();

        for pull in pulls {
            let balls: Vec<_> = pull.split(", ").collect();
            for ball in balls {
                let b = &ball_regex.captures(&ball).unwrap();    
                if bag.get(&b[2]).unwrap() < &b[1].parse::<i32>().unwrap() {
                    possible = false;
                    break;
                }
            }
        }
        if possible {
            ans = ans + game_number;
            println!("{}", game_number)
        }
    }
    println!("{}", ans)
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}