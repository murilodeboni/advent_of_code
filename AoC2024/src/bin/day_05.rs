mod utils;

use utils::input::read_lines;

use std::collections::{HashMap, HashSet};

fn main() {
    let ordering_rules = read_lines("./src/bin/inputs/day_05_01.txt");
    let manual: Vec<String> = read_lines("./src/bin/inputs/day_05_02.txt");
    // let manual: Vec<String> = vec!["75,97,47,61,53".to_string()];

    let mut map: HashMap<usize, HashSet<usize>> = HashMap::new();

    let mut part1Ans: usize = 0;
    let mut part2Ans: usize = 0;

    for i in ordering_rules {
        let parts: Vec<&str> = i.split("|").collect();
        let p1: usize = parts[0].parse().expect("error");
        let p2: usize = parts[1].parse().expect("error");

        if let Some(values) = map.get_mut(&p2) {
            values.insert(p1); 
        } else {
            map.entry(p2).or_default().insert(p1);
        }
    }

    for l in manual {
        let manual_pages_string: Vec<&str> = l.split(",").collect();
        let manual_pages: Vec<usize> = manual_pages_string.iter().map(|l| l.parse::<usize>().expect("Failed to parse")).collect();
        let is_ordered = part1(&manual_pages, &map);
        part1Ans += is_ordered;
        part2Ans += if is_ordered == 0 {part2(&manual_pages, &map)} else {0};
    }

    println!("{}", part1Ans);
    println!("{}", part2Ans);

}

fn part2(manual_pages: &Vec<usize>, map: &HashMap<usize,HashSet<usize>>) -> usize {
    let mut new_pages = manual_pages.clone();
    new_pages.sort_by(|a, b| map[b].contains(a).cmp(&true));
    get_middle_page(&new_pages)
}

fn part1(manual_pages: &Vec<usize>, map: &HashMap<usize,HashSet<usize>>) -> usize {
    let new_pages = manual_pages.clone();
    if new_pages.is_sorted_by(|a, b| map[b].contains(a)) {
        get_middle_page(&new_pages)
    } else {
        0
    }
}

fn get_middle_page(v:&Vec<usize>) -> usize {
    v[(v.len()-1)/2]
}
