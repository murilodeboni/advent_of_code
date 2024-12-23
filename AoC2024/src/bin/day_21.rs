mod utils;

use std::collections::HashMap;

use utils::input::read_lines;

#[derive(Clone)]
struct Key {
    position: (usize, usize)
}

#[derive(Clone)]
struct KeyPad {
    current: char,
    dists: HashMap<(char,char), String>
}

impl KeyPad {
    fn reset(&mut self) {
        self.current = 'A'
    }

    fn invert(&self, ch: char) -> char {
        match ch {
            '^' => 'v',
            'v' => '^',
            '<' => '>',
            '>' => '<',
            _ => ch
        }
    }

    fn invert_and_mirror(&self, s: &str) -> String {
        s.chars()
            .rev()
            .map(|c| self.invert(c))
            .collect()
    }
    
    fn get_expanded(&self, a: char, b: char) -> String {
        if let Some(val) = self.dists.get(&(a, b)) {
            return val.clone()
        } else if let Some(val) = self.dists.get(&(b, a)) {
            return self.invert_and_mirror(val)
        } else if a == b {
            return String::new();
        } else {
            println!("PROBLEM {} {}", a,b);
            return String::new();
        }
    }

    fn dist(&self, c: char) -> String {
        self.get_expanded(self.current, c)
    }

    fn type_command(&mut self, input: Vec<char>) -> String {
        let mut s = "".to_owned();
        for c in input {
            s.push_str(&self.dist(c));
            s.push_str("A");
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
        dists: HashMap::from([
            (('A','0'), "<".to_string()),
            (('A','1'), "^<<".to_string()),
            (('A','2'), "^<".to_string()),
            (('A','3'), "^".to_string()),
            (('A','4'), "<<^^".to_string()),
            (('A','5'), "^^<".to_string()),
            (('A','6'), "^^".to_string()),
            (('A','7'), "<<^^^".to_string()),
            (('A','8'), "^^^<".to_string()),
            (('A','9'), "^^^".to_string()),
            (('0','1'), "^<".to_string()),
            (('0','2'), "^".to_string()),
            (('0','3'), "^>".to_string()),
            (('0','4'), "^^<".to_string()),
            (('0','5'), "^^".to_string()),
            (('0','6'), "^^>".to_string()),
            (('0','7'), "^^^<".to_string()),
            (('0','8'), "^^^".to_string()),
            (('0','9'), "^^^>".to_string()),
            (('1','2'), ">".to_string()),
            (('1','3'), ">>".to_string()),
            (('1','4'), "^".to_string()),
            (('1','5'), "^>".to_string()),
            (('1','6'), "^>>".to_string()),
            (('1','7'), "^^".to_string()),
            (('1','8'), "^^>".to_string()),
            (('1','9'), "^^>>".to_string()),
            (('2','3'), ">".to_string()),
            (('2','4'), "^<".to_string()),
            (('2','5'), "^".to_string()),
            (('2','6'), "^>".to_string()),
            (('2','7'), "^^<".to_string()),
            (('2','8'), "^^".to_string()),
            (('2','9'), "^^>".to_string()),
            (('3','4'), "^<<".to_string()),
            (('3','5'), "^<".to_string()),
            (('3','6'), "^".to_string()),
            (('3','7'), "<<^^".to_string()),
            (('3','8'), "^^<".to_string()),
            (('3','9'), "^^".to_string()),
            (('4','5'), ">".to_string()),
            (('4','6'), ">>".to_string()),
            (('4','7'), "^".to_string()),
            (('4','8'), "^>".to_string()),
            (('4','9'), "^>>".to_string()),
            (('5','6'), ">".to_string()),
            (('5','7'), "^<".to_string()),
            (('5','8'), "^".to_string()),
            (('5','9'), "^>".to_string()),
            (('6','7'), "^<<".to_string()),
            (('6','8'), "^<".to_string()),
            (('6','9'), "^".to_string()),
            (('7','8'), ">".to_string()),
            (('7','9'), ">>".to_string()),
            (('8','9'), ">".to_string())
        ])
    };

    let mut directionalPad = KeyPad{
        current:'A',
        dists: HashMap::from([
            (('A','>'), "v".to_string()),
            (('A','v'), "v<".to_string()),
            (('A','<'), "v<<".to_string()),
            (('A','^'), "<".to_string()),
            (('>','v'), "<".to_string()),
            (('>','<'), "<<".to_string()),
            (('>','^'), "^<".to_string()),
            (('v','<'), "<".to_string()),
            (('v','^'), "^".to_string()),
            (('<','^'), ">^".to_string()),
        ])
    };

    let mut directionalPad2 = directionalPad.clone();

    let input = read_lines("./src/bin/inputs/day_21.txt");
    let commands: Vec<Vec<char>> = input.iter().map(|s| s.chars().collect()).collect();

    let mut part1 = 0;

    for command in commands {
        let n = vec_to_number(&command);
        println!("{:?}", command);
        // numberPad.reset();
        // directionalPad.reset();
        // directionalPad2.reset();

        println!("{} {} {}", numberPad.current, directionalPad.current, directionalPad2.current);
        let level1 = numberPad.type_command(command);
        println!("{:?}", level1);
        let level2 = directionalPad.type_command(level1.chars().collect());
        println!("{:?}", level2);
        let level3 = directionalPad2.type_command(level2.chars().collect());
        part1 += n*level3.len();
        println!("{} {} {}", level3.len(), n, level3);
    }

    println!("{}", part1)

}