mod utils;

use utils::input::read_lines;

use std::{collections::HashMap, time::Instant};

struct SecretNumber{
    value: usize,
    last_digits: Vec<usize>,
    price_changes: HashMap<(isize,isize,isize,isize),usize>
}

impl SecretNumber {
    fn mix(&mut self, value: usize) {
        //calculate the bitwise XOR of the given value and the secret number
        self.value = value ^ self.value;
    }
    fn prune(&mut self) {
        //calculate the value of the secret number modulo 16777216
        self.value = self.value % 16777216;
    }
    fn next(&mut self) {
        // step 1
        let s1 = self.value * 64;
        self.mix(s1);
        self.prune();

        //step 2
        let s2 = self.value / 32;
        self.mix(s2);
        self.prune();

        //step 3
        let s3 = self.value * 2048;
        self.mix(s3);
        self.prune();

        self.add_last_digit();
    }

    fn add_last_digit(&mut self) {
        self.last_digits.push(self.value % 10);
    }

    fn find_change_seqs(&mut self) {
        let mut i = 4;
        while i < self.last_digits.len() {
            let key: (isize,isize,isize,isize) = (
                self.last_digits[i] as isize - self.last_digits[i-1] as isize,
                self.last_digits[i-1] as isize - self.last_digits[i-2] as isize,
                self.last_digits[i-2] as isize - self.last_digits[i-3] as isize,
                self.last_digits[i-3] as isize - self.last_digits[i-4] as isize
            );
            self.price_changes.entry(key).or_insert(self.last_digits[i]);
            i += 1;
        }
    }
}

fn main() {
    let start_time = Instant::now();
    let secrets: Vec<usize> = read_lines("./src/bin/inputs/day_22.txt").iter().map(|s| s.parse().expect("error")).collect();

    let mut part1: usize  = 0;
    let mut changes: Vec<HashMap<(isize,isize,isize,isize),usize>> = Vec::new();

    for secret in secrets {
        // println!("starting to calculate 2000th element for {}", secret);
        let mut secret_number = SecretNumber{value: secret, last_digits: Vec::new(), price_changes: HashMap::new()};
        secret_number.add_last_digit();
        let mut i = 0;
        while i < 2000 {
            // println!("Nø {} - {}, last digit {}", i, secret_number.value, secret_number.last_digit());
            secret_number.next();
            i += 1;
        }
        part1 += secret_number.value;
        secret_number.find_change_seqs();
        changes.push(secret_number.price_changes);
    }

    println!("part 1 - {} took {}ms", part1, start_time.elapsed().as_millis());
    
    let part2 = part2(changes);

    println!("part 2 - {} took {}ms", part2, start_time.elapsed().as_millis());
}

fn part2(changes: Vec<HashMap<(isize,isize,isize,isize),usize>>) -> usize {
    let mut combined: HashMap<(isize, isize, isize, isize), usize> = HashMap::new();

    for change in changes {
        for (key, value) in change {
            *combined.entry(key).or_insert(0) += value;
        }
    }

    let mut max_value: &usize = &0;

    for (_, value) in &combined {
        if value > &max_value {
            max_value = value;
        }
    }

    *max_value
}