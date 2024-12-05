mod utils;

use utils::input::read_lines;

use std::{collections::HashMap, vec};


fn main() {
    let ordering_rules = read_lines("./src/bin/inputs/day_05_01.txt");
    let manual: Vec<String> = read_lines("./src/bin/inputs/day_05_02.txt");
    // let manual: Vec<String> = vec!["75,97,47,61,53".to_string()];

    let mut map: HashMap<usize, Vec<usize>> = HashMap::new();

    let mut part1: usize = 0;

    for i in ordering_rules {
        let parts: Vec<&str> = i.split("|").collect();
        let p1: usize = parts[0].parse().expect("error");
        let p2: usize = parts[1].parse().expect("error");

        if let Some(values) = map.get_mut(&p2) {
            values.push(p1); 
        } else {
            map.entry(p2).or_insert_with(|| vec![p1]);
        }
    }

    for l in manual {
        let manual_pages_string: Vec<&str> = l.split(",").collect();
        let manual_pages: Vec<usize> = manual_pages_string.iter().map(|l| l.parse::<usize>().expect("Failed to parse")).collect();
        let middle_page = get_middle_page(&manual_pages);
        
        let mut follow_rules: bool = true;

        let mut previous_page = &manual_pages[0];

        for page in &manual_pages[1..] {
            if !follow_rules {
                continue;
            }
            if let Some(possibilities) = map.get(&page) {
                follow_rules = check_if_pages_follow_rules(&previous_page, possibilities);
            } else {
                follow_rules = false;
            }
            previous_page = &page;
        }
        if follow_rules {
            part1 += middle_page;
        }
    }

    println!("{}", part1);

}

fn get_middle_page(v:&Vec<usize>) -> usize {
    v[(v.len()-1)/2]
}

fn check_if_pages_follow_rules(previous_page: &usize, possibilities: &Vec<usize>) -> bool {
    let ok: bool = possibilities.contains(&previous_page);
    // println!("{} {:?} {:?}", ok, possibilities, previous_page);
    ok
}