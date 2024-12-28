mod utils;

use utils::input::read_lines;

use std::{collections::{HashMap, HashSet}, hash::Hash, time::Instant};

fn main() {
    let start_time = Instant::now();
    let input: Vec<String> = read_lines("./src/bin/inputs/day_23.txt");

    let mut connections: HashMap<String, HashSet<String>> = HashMap::new();

    for conn in input {
        let pcs: Vec<&str> = conn.split("-").collect();
        connections.entry(pcs[0].to_string()).or_insert(HashSet::new()).insert(pcs[1].to_string());
        connections.entry(pcs[1].to_string()).or_insert(HashSet::new()).insert(pcs[0].to_string());
    }

    let mut part1: usize  = part1(connections);

    println!("part 1 - {} took {}ms", part1, start_time.elapsed().as_millis());
}

fn part1(connections: HashMap<String, HashSet<String>>) -> usize {
    let mut part1: usize  = 0;

    let mut three_way_connections: HashSet<Vec<String>> = HashSet::new();

    // the computer
    for (computer, neighbors) in &connections {
        // first connection
        for neighbor in neighbors {
            let mut nn = connections.get(neighbor).unwrap();
            // second level connection
            for neighbor2 in nn {
                if neighbor2 == computer {
                    continue;
                }
                let nn2 = connections.get(neighbor2).unwrap();
                if let Some(c) = nn2.get(computer) {
                    let mut v = vec![computer.clone(), neighbor.clone(), neighbor2.clone()];
                    v.sort();
                    three_way_connections.insert(v);
                }
            }
        }
    }

    for cs in &three_way_connections {
        if cs[0].starts_with("t") || cs[1].starts_with("t") || cs[2].starts_with("t") {
            part1 += 1;
        }
    }

    // println!("{:?}", three_way_connections);

    part1
}