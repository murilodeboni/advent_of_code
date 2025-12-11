use std::collections::{BTreeSet, HashSet};
use std::time::Instant;
use aoc_utils::read_input;

const BASE: &str = env!("CARGO_MANIFEST_DIR");
const DAY: &str = "d08";

#[derive(Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Juction {
    x: usize,
    y: usize,
    z: usize,
}

impl Juction {
    fn euclidean_distance(&self, other: &Juction) -> f64 {
        let dx = (self.x as isize - other.x as isize) as f64;
        let dy = (self.y as isize - other.y as isize) as f64;
        let dz = (self.z as isize - other.z as isize) as f64;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
struct Circuit {
    juctions: BTreeSet<Juction>
}

fn compare_juctions(junctions: &[Juction]) -> Vec<(f64, Juction, Juction)> {
    let mut edges = Vec::new();
    for i in 0..junctions.len() {
        for j in (i + 1)..junctions.len() {
            let dist = junctions[i].euclidean_distance(&junctions[j]);
            edges.push((dist, junctions[i].clone(), junctions[j].clone()));
        }
    }
    edges.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    edges
}

fn create_circuits(edges: &[(f64, Juction, Juction)], circuits: &mut HashSet<Circuit>, attempts: usize) {
    for (_, j1, j2) in edges.iter().take(attempts) {
        let c1 = circuits.iter().find(|c| c.juctions.contains(j1)).cloned();
        let c2 = circuits.iter().find(|c| c.juctions.contains(j2)).cloned();

        match (c1, c2) {
            (None, None) => {
                let mut new_circuit = Circuit { juctions: BTreeSet::new() };
                new_circuit.juctions.insert(j1.clone());
                new_circuit.juctions.insert(j2.clone());
                circuits.insert(new_circuit);
            }
            (Some(circ), None) => {
                if let Some(mut circuit) = circuits.take(&circ) {
                    circuit.juctions.insert(j2.clone());
                    circuits.insert(circuit);
                }
            }
            (None, Some(circ)) => {
                if let Some(mut circuit) = circuits.take(&circ) {
                    circuit.juctions.insert(j1.clone());
                    circuits.insert(circuit);
                }
            }
            (Some(c1), Some(c2)) => {
                if c1 == c2 {
                    // already connected
                    continue;
                }
                let mut merged = circuits.take(&c1).unwrap_or_else(|| Circuit { juctions: BTreeSet::new() });
                if let Some(other) = circuits.take(&c2) {
                    merged.juctions.extend(other.juctions.into_iter());
                }
                circuits.insert(merged);
            }
        }
    }
}

fn create_one_big_circuit(
    edges: &Vec<(f64, Juction, Juction)>
    , junctions: &[Juction]
    , circuits: &mut HashSet<Circuit>
) -> (Juction, Juction) {
    let mut remaining = circuits.len();

    for (_, j1, j2) in edges {
        let c1 = circuits.iter().find(|c| c.juctions.contains(&j1)).cloned();
        let c2 = circuits.iter().find(|c| c.juctions.contains(&j2)).cloned();

        match (c1, c2) {
            (Some(a), Some(b)) if a == b => {
                // already connected
                continue;
            }
            (Some(a), Some(b)) => {
                let mut merged = circuits
                    .take(&a)
                    .unwrap_or_else(|| Circuit { juctions: BTreeSet::new() });
                if let Some(other) = circuits.take(&b) {
                    merged.juctions.extend(other.juctions.into_iter());
                }
                circuits.insert(merged);
                remaining -= 1;
            }
            (Some(a), None) => {
                if let Some(mut circuit) = circuits.take(&a) {
                    circuit.juctions.insert(j2.clone());
                    circuits.insert(circuit);
                    remaining -= 1;
                }
            }
            (None, Some(b)) => {
                if let Some(mut circuit) = circuits.take(&b) {
                    circuit.juctions.insert(j1.clone());
                    circuits.insert(circuit);
                    remaining -= 1;
                }
            }
            (None, None) => {
                let mut new_circuit = Circuit { juctions: BTreeSet::new() };
                new_circuit.juctions.insert(j1.clone());
                new_circuit.juctions.insert(j2.clone());
                circuits.insert(new_circuit);
                remaining -= 1;
            }
        }

        if remaining == 1 {
            return (j1.clone(), j2.clone());
        }
    }

    panic!("Failed to connect all junctions");
}

fn parse_input(input: Vec<String>) -> Vec<Juction> {
    let mut juctions = Vec::new();

    for line in input {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() == 3 {
            if let (Ok(x), Ok(y), Ok(z)) = (parts[0].parse(), parts[1].parse(), parts[2].parse()) {
                juctions.push(Juction { x, y, z });
            }
        }
    }
    juctions
}

fn ans(circuits: &HashSet<Circuit>, n: usize) -> usize {
    let mut total = 1;
    let mut largest_circuits: Vec<&Circuit> = circuits.iter().collect();
    largest_circuits.sort_by_key(|c| usize::MAX - c.juctions.len());
    for circuit in largest_circuits.iter().take(n) {
        total *= circuit.juctions.len();
    }
    total
}

fn main() {
    let start = Instant::now();

    let input = read_input(BASE, DAY, false);

    let junctions = parse_input(input);
    
    let mut circuits: HashSet<Circuit> = junctions
        .iter()
        .map(|j| {
            let mut set = BTreeSet::new();
            set.insert(j.clone());
            Circuit { juctions: set }
        })
        .collect();

    let edges = compare_juctions(&junctions);

    create_circuits(&edges, &mut circuits, 1000);

    let part1 = ans(&circuits, 3);

    let (j1, j2) = create_one_big_circuit(&edges, &junctions, &mut circuits);
    
    println!("{DAY} part1: {}", part1);
    println!("{DAY} part2: {}", j1.x*j2.x);
    println!("Elapsed: {}ms", start.elapsed().as_millis());

}
