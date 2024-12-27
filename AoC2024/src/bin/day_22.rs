mod utils;

use utils::input::read_lines;

use std::time::Instant;

struct SecretNumber{
    value: usize
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
    }
}

fn main() {
    let start_time = Instant::now();
    let secrets: Vec<usize> = read_lines("./src/bin/inputs/day_22.txt").iter().map(|s| s.parse().expect("error")).collect();

    let mut part1: usize  = 0;

    let mut test_mix = SecretNumber{value: 42};
    test_mix.mix(15);
    println!("mixing works - {:?}", test_mix.value == 37);

    let mut test_prune = SecretNumber{value: 100000000};
    test_prune.prune();
    println!("pruning works - {}", test_prune.value == 16113920);

    for secret in secrets {
        println!("starting to calculate 2000th element for {}", secret);
        let mut secret_number = SecretNumber{value: secret};
        let mut i = 0;
        while i < 2000 {
            secret_number.next();
            i += 1;
        }
        println!("Nø {} - {}", i, secret_number.value);
        part1 += secret_number.value;
    }
    
    println!("part 1 - {} took {}ms", part1, start_time.elapsed().as_millis());
}