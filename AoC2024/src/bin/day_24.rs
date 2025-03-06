mod utils;

use utils::input::read_lines;

use std::{collections::HashMap, time::Instant};

struct Operation {
    w1: String,
    w2: String,
    wn: String,
    operation: String,
    resolved: bool
}

impl Operation {
    fn calculate(&mut self, wires: &mut HashMap<String,u8>) {
        if !wires.contains_key(&self.w1) || !wires.contains_key(&self.w2) {
            self.resolved = false;
        } else if self.operation == "AND" {
            let w1 = wires.get(&self.w1).expect("error");
            let w2 = wires.get(&self.w2).expect("error");
            wires.insert(self.wn.to_string(), w1 & w2);
            self.resolved = true;
        } else if self.operation == "OR" {
            let w1 = wires.get(&self.w1).expect("error");
            let w2 = wires.get(&self.w2).expect("error");
            wires.insert(self.wn.to_string(), w1 | w2);
            self.resolved = true;
        } else if self.operation == "XOR" {
            let w1 = wires.get(&self.w1).expect("error");
            let w2 = wires.get(&self.w2).expect("error");
            wires.insert(self.wn.to_string(), w1 ^ w2);
            self.resolved = true;
        }
    }
}

fn binary_vec_to_number(binary_vec: Vec<&u8>) -> u64 {
    binary_vec.iter().enumerate().fold(0, |acc, (i, &bit)| {
        acc + ((*bit as u64) << i)
    })
}


fn main() {
    let start_time = Instant::now();

    let input_1: Vec<(String,String)> = read_lines("./src/bin/inputs/day_24_1.txt").iter().map(|s| {
        let parts: Vec<&str> = s.split(": ").collect();
        (parts[1].to_string(), parts[0].to_string())
    }).collect();

    let mut wires: HashMap<String,u8> = input_1.iter().map(|(v,k)| (k.to_string(),v.parse().expect("error"))).collect();

    let mut operations: Vec<Operation> = read_lines("./src/bin/inputs/day_24_2.txt").iter()
    .map(
        |s| {
        let parts: Vec<&str> = s.split(" -> ").collect();
        let first_parts: Vec<&str> = parts[0].split(" ").collect();
        Operation{w1: first_parts[0].to_string(), operation: first_parts[1].to_string(), w2: first_parts[2].to_string(), wn: parts[1].to_string(), resolved: false}
    }).collect();

    while operations.iter().any(|o| !o.resolved) {
            for op in operations.iter_mut().filter(|o| !o.resolved) {
            op.calculate(&mut wires);
        }
    }

    let mut part1: Vec<(String, u8)> = Vec::new();

    for (k,v) in wires {
        part1.push((k,v));
    }

    part1.sort_by(|a,b| a.0.cmp(&b.0));
    part1 = part1.into_iter().filter(|(k,_)| k.starts_with("z")).collect();
    let part1_ans: Vec<&u8> = part1.iter().map(|(_,v)| v).collect();

    println!("part 1 - {:?} took {}ms", binary_vec_to_number(part1_ans), start_time.elapsed().as_millis());
}