mod utils;

use utils::input::read_lines;

use std::{collections::{HashMap, HashSet}, time::Instant};

fn main() {
    let start_time = Instant::now();
    let input: Vec<String> = read_lines("./src/bin/inputs/day_23.txt");

    let mut connections: HashMap<String, HashSet<String>> = HashMap::new();

    for conn in input {
        let pcs: Vec<&str> = conn.split("-").collect();
        connections.entry(pcs[0].to_string()).or_insert(HashSet::new()).insert(pcs[1].to_string());
        connections.entry(pcs[1].to_string()).or_insert(HashSet::new()).insert(pcs[0].to_string());
    }

    let part1_ans = part1(&connections);

    let part2 = find_all_cliques(&connections);

    let mut part2_ans: Vec<String> = Vec::new();
    for c in part2 {
        part2_ans.push(c);
    }
    part2_ans.sort();
    
    println!("part 1 - {} took {}ms", part1_ans, start_time.elapsed().as_millis());
    println!("part 2 - {} took {}ms", part2_ans.join(","), start_time.elapsed().as_millis());

}

fn part1(connections: &HashMap<String, HashSet<String>>) -> usize {
    let mut part1: usize  = 0;

    let mut three_way_connections: HashSet<Vec<String>> = HashSet::new();

    // the computer
    for (computer, neighbors) in connections {
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
    part1
}

fn find_all_cliques(graph: &HashMap<String, HashSet<String>>) -> HashSet<String> {
    let mut cliques = Vec::new();
    let mut p: HashSet<_> = graph.keys().cloned().collect();
    let mut r = HashSet::new();
    let mut x = HashSet::new();

    bron_kerbosch(graph, &mut r, &mut p, &mut x, &mut cliques);

    let mut biggest_clique: HashSet<String> = HashSet::new();

    for clique in cliques {
        if &clique.len() > &biggest_clique.len() && &clique.len() >= &3 {
            biggest_clique = clique.clone();
        }
    }

    biggest_clique
}

fn bron_kerbosch(
    graph: &HashMap<String, HashSet<String>>,
    r: &mut HashSet<String>,
    p: &mut HashSet<String>,
    x: &mut HashSet<String>,
    cliques: &mut Vec<HashSet<String>>,
) {
    if p.is_empty() && x.is_empty() {
        // Found a maximal clique
        cliques.push(r.clone());
        return;
    }

    let p_clone = p.clone();
    for v in p_clone {
        r.insert(v.clone()); // Add v to the current clique

        // Compute new p and x: intersection with neighbors of v
        let neighbors = graph.get(&v).unwrap();
        let mut new_p: HashSet<_> = p.intersection(neighbors).cloned().collect();
        let mut new_x: HashSet<_> = x.intersection(neighbors).cloned().collect();

        // Recur with updated sets
        bron_kerbosch(graph, r, &mut new_p, &mut new_x, cliques);

        // Backtrack: remove v from r and move it from p to x
        r.remove(&v);
        p.remove(&v);
        x.insert(v);
    }
}
