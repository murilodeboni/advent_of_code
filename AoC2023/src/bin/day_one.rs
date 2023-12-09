use std::collections::HashMap;
use std::fs::read_to_string;
use regex::Regex;

fn main() {
    let input = read_lines("/Users/murilodeboni/advent_of_code/AoC2023/src/bin/day_one_input.txt");

    let digits_map = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
        ("zero", "0")
    ]);

    

    let first_re = Regex::new(r"^.*?(\d|one|two|three|four|five|six|seven|eight|nine|zero}]).*$").unwrap();
    let last_re = Regex::new(r".*(\d|one|two|three|four|five|six|seven|eight|nine|zero}]).*$").unwrap();

    let mut ans = 0;

    for i in input {
        let Some(first_digit) = first_re.captures(&i) else { return };
        let Some(last_digit) = last_re.captures(&i) else { return };

        let first: &str = digits_map.get(&&first_digit[1]).unwrap_or(&&first_digit[1]);
        let last: &str = digits_map.get(&&last_digit[1]).unwrap_or(&&last_digit[1]);

        ans = &ans + [first,last].join("").parse::<i32>().unwrap();
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