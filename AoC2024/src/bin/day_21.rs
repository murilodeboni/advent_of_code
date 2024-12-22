mod utils;

use std::{collections::HashMap, hash::Hash};

use utils::input::read_lines;

#[derive(Clone)]
struct Key {
    position: (usize, usize)
}

#[derive(Clone)]
struct KeyPad {
    current: char,
    keys: HashMap<char, Key>
}

impl KeyPad {
    fn reset(&mut self) {
        self.current = 'A'
    }

    fn dist(&self, c: char) -> String {
        let (y1,x1) = self.keys.get(&self.current).unwrap().position;
        let (y2,x2) = self.keys.get(&c).unwrap().position;
        let dx = x2 as isize - x1 as isize;
        let dy = y2 as isize - y1 as isize;

        // println!("dx {} dy {}", dx, dy);

        let mut s = "".to_owned();
        if dx > 0 {
            let rights: String = std::iter::repeat('>').take(dx as usize).collect();
            s.push_str(&rights);
        } else if dx < 0 {
            let lefts: String = std::iter::repeat('<').take(dx.abs() as usize).collect();
            s.push_str(&lefts);
        }

        if dy > 0 {
            let ups: String = std::iter::repeat('v').take(dy as usize).collect();
            s.push_str(&ups);
        } else if dy < 0 {
            let downs: String = std::iter::repeat('^').take(dy.abs() as usize).collect();
            s.push_str(&downs);
        }
        s.push_str("A");
        // println!("{}", s);

        s.to_string()
    }

    fn type_command(&mut self, input: Vec<char>) -> String {
        let mut s = "".to_owned();
        for c in input {
            s.push_str(&self.dist(c));
            self.current = c;
        }
        s
    }
}

fn vec_to_number(chars: &Vec<char>) -> usize {
    let numeric_str: String = chars
        .into_iter()
        .filter(|c| c.is_digit(10)) // Keep only numeric characters
        .collect();

    numeric_str.parse::<usize>().ok().unwrap()
}

fn main() {
    let mut numberPad = KeyPad{
        current: 'A',
        keys: HashMap::from([
            ('A', Key{position: (3,2)}),
            ('0', Key{position: (3,1)}),
            ('1', Key{position: (2,0)}),
            ('2', Key{position: (2,1)}),
            ('3', Key{position: (2,2)}),
            ('4', Key{position: (1,0)}),
            ('5', Key{position: (1,1)}),
            ('6', Key{position: (1,2)}),
            ('7', Key{position: (0,0)}),
            ('8', Key{position: (0,1)}),
            ('9', Key{position: (0,2)}),
        ])
    };

    let mut directionalPad = KeyPad{
        current:'A',
        keys: HashMap::from([
            ('A', Key{position: (0,2)}),
            ('^', Key{position: (0,1)}),
            ('<', Key{position: (1,0)}),
            ('v', Key{position: (1,1)}),
            ('>', Key{position: (1,2)}),
        ])
    };

    let mut directionalPad2 = directionalPad.clone();

    let input = read_lines("./src/bin/inputs/day_21_test.txt");
    let commands: Vec<Vec<char>> = input.iter().map(|s| s.chars().collect()).collect();

    let mut part1 = 0;

    for command in commands {
        let n = vec_to_number(&command);
        println!("{:?}", command);
        // numberPad.reset();
        // directionalPad.reset();
        // directionalPad2.reset();

        // println!("{} {} {}", numberPad.current, directionalPad.current, directionalPad2.current);
        let level1 = numberPad.type_command(command);
        // println!("{:?}", level1);
        let level2 = directionalPad.type_command(level1.chars().collect());
        // println!("{:?}", level2);
        let level3 = directionalPad2.type_command(level2.chars().collect());
        part1 += n*level3.len();
        println!("{} {} {}", level3.len(), n, level3);
    }

    println!("{}", part1)

}