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

    let mut part1: usize  = part1(&connections);

    println!("part 1 - {} took {}ms", part1, start_time.elapsed().as_millis());

    let mut network  = part2(&connections);
    let mut part2: Vec<String> = Vec::new();
    for c in network {
        part2.push(c);
    }
    part2.sort();

    println!("part 2 - {} took {}ms", part2.join(","), start_time.elapsed().as_millis());

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

    // println!("{:?}", three_way_connections);
    
    part1
}

fn part2(graph: &HashMap<String, HashSet<String>>) -> HashSet<String> {
    let mut cliques = Vec::new();
    let mut p: HashSet<_> = graph.keys().cloned().collect();
    let mut r = HashSet::new();
    let mut x = HashSet::new();

    bron_kerbosch(graph, &mut r, &mut p, &mut x, &mut cliques);

    // Find the largest clique
    cliques.into_iter().max_by_key(|clique| clique.len()).unwrap_or_default()
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
    } else {
        let pivot = p.union(x).next().unwrap().clone(); // Choose a pivot
        let mut p_without_neighbors: HashSet<_> = p.difference(&graph[&pivot]).cloned().collect();

        for v in p_without_neighbors {
            // Add v to the current clique
            r.insert(v.clone());

            // Recur with neighbors of v
            let neighbors = &graph[&v];
            let mut p_intersection: HashSet<_> = p.intersection(neighbors).cloned().collect();
            let mut x_intersection: HashSet<_> = x.intersection(neighbors).cloned().collect();
            bron_kerbosch(graph, r, &mut p_intersection, &mut x_intersection, cliques);

            // Backtrack
            r.remove(&v);
            p.remove(&v);
            x.insert(v);
        }
    }
}
