mod utils;

use utils::input::read_lines;

use std::{collections::{HashMap, HashSet}, time::Instant};

#[derive(Debug, PartialEq)]
struct KeyLock {
    pins: Vec<usize>
}

impl KeyLock {
    fn fits_lock(&self, lock: &KeyLock) -> bool {
        for i in 0..5 {
            if self.pins[i] + lock.pins[i] > 5 {
                // println!("position {} -- {} > {}", i, self.pins[i], lock.pins[i]);
                return false;
            }
        }
        true
    }
}

fn main() {
    let start_time = Instant::now();
    let input: Vec<String> = read_lines("./src/bin/inputs/day_25.txt");

    let mut input_split: Vec<Vec<String>> = Vec::new();

    let mut partial: Vec<String> = Vec::new();
    for i in input {
        if i == "" {
            input_split.push(partial.clone());
            partial.clear();
        } else {
            partial.push(i);
        }
    }
    input_split.push(partial.clone());

    let mut keys: Vec<KeyLock> = Vec::new();
    let mut locks: Vec<KeyLock> = Vec::new();

    for p in input_split {
        let (locktype, kl) = parse_key_lock(&p);
        if locktype == "key" {
            keys.push(kl);
        } else  if locktype == "lock"{
            locks.push(kl);
        } else {
            panic!("Unknown lock type");
        }
    }

    let part1: usize = part1(&keys, &locks);
    println!("part 1 - {} took {}ms", part1, start_time.elapsed().as_millis());
}

fn get_pins(p: &Vec<String>)  -> Vec<usize> {
    let l: usize = p.len();
    let mut pins: Vec<usize> = vec![0; 5];

    for i in 1..l {
        for (n,c) in p[i].chars().enumerate() {
            if c == '#' {
                pins[n] = pins[n]+ 1;
            }
        }
    }

    pins
}

fn parse_key_lock(p: &Vec<String>) -> (&str, KeyLock) {
    if p[0] == "#####".to_string() {
        return ("lock", KeyLock{pins: get_pins(p)});
    } else {
        let mut pr = p.clone();
        pr.reverse();
        return ("key", KeyLock{pins: get_pins(&pr)})
    }
}

fn part1(keys: &Vec<KeyLock>, locks: &Vec<KeyLock>) -> usize {
    let mut part1: usize = 0;
    for k in keys {
        for l in locks {
            // println!("Trying key {:?} with lock {:?}", k, l);
            if k.fits_lock(l) {
                part1 += 1;
            }
        }
    }
    part1
}