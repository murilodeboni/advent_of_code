mod utils;

use std::fmt;
use std::hash::Hash;

use utils::input::read_lines;

use std::collections::{HashMap, HashSet};

use std::time::Instant;

#[derive(Eq, Hash, PartialEq)]
struct Node {
    x: usize,
    y: usize
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Node({}, {})", self.x, self.y)
    }
}

impl Node {
    fn find_antinodes(&self, node: &Node, max_rows: isize) -> Vec<Node> {
        let dx: isize = self.x as isize - node.x as isize;
        let dy: isize = self.y as isize - node.y as isize;

        let mut ans: Vec<Node> = vec![];

        let mut d1 = (self.x as isize + dx, self.y as isize + dy);
        let mut d2 = (node.x as isize - dx, node.y as isize - dy);

        // PART 1 is using if instead of while
        while check_boundaries(d1, max_rows) {
            ans.push(Node{x: d1.0 as usize, y: d1.1 as usize});
            d1 = (d1.0 + dx, d1.1 + dy);
        }
        while check_boundaries(d2, max_rows) {
            ans.push(Node{x: d2.0 as usize, y: d2.1 as usize});
            d2 = (d2.0 - dx, d2.1 - dy);
        }
        ans
    }
}

fn main() {
    let start = Instant::now();
    let input = read_lines("./src/bin/inputs/day_08.txt");
    let mut ans: usize = 0;

    let mut node_map: HashMap<char, Vec<Node>> = HashMap::new();

    for j in 0..input.len() {
        let line = &input[j];
        let chars: Vec<char> = line.chars().collect();
        for i in 0..chars.len() {
            let c = chars[i];
            if c.is_alphanumeric() {
                if let Some(values) = node_map.get_mut(&c) {
                    values.push(Node{x:i, y:j}); 
                } else {
                    node_map.entry(c).or_insert_with(|| vec![Node{x:i, y:j}]);
                }
            }
        }
    }

    ans += part1(node_map, &input);
    println!("Answer - {}, took {}ms", ans, start.elapsed().as_millis());


}

fn part1(node_map: HashMap<char, Vec<Node>>, input: &Vec<String>) -> usize {
    let max_rows = input.len() as isize;

    let mut unique: HashSet<Node> = HashSet::new();
    
    for (c, nodes) in &node_map {
        for i in 0..nodes.len() {
            for j in (i+1)..nodes.len() {
                let an = nodes[i].find_antinodes(&nodes[j], max_rows);
                // debug_grid(max_rows as usize, c, vec![&nodes[i],&nodes[j]], &an);
                unique.extend(an);
            }
        }

    }

    for (c, ns) in node_map {
        if &ns.len() > &1 {
            unique.extend(ns);
        }
    }

    unique.len()
}

fn check_boundaries(node: (isize, isize), max_rows: isize) -> bool {
    node.0 as isize >= 0 && node.1 as isize >= 0 && (node.0 as isize) < max_rows && (node.1 as isize) <  max_rows
}

fn debug_grid(n: usize, ch: char, locations: Vec<&Node>, antinodes: &Vec<Node>) {
    let mut grid = vec![vec!['.'; n]; n];

    for loc in &locations {
        if loc.x < n && loc.y < n {
            grid[loc.y][loc.x] = ch;
        }
    }

    for anti in antinodes {
        if anti.x < n && anti.y < n {
            grid[anti.y][anti.x] = '#';
        }
    }

    for row in grid {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
    println!("======")
}