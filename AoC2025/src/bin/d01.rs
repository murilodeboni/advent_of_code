use aoc_utils::read_input;

const BASE: &str = env!("CARGO_MANIFEST_DIR");
const DAY: &str = "d01";

struct DirectionInput {
    direction: char,
    value: i64,
}

struct Dial {
    position: i64,
    part1: i64,
    part2: i64,
} 

impl Dial {
    fn new() -> Self {
        Dial { position: 50 , part1: 0, part2: 0 }
    }

    fn turn1(&mut self, direction: char, value: i64) {
        let initial_position = self.position.clone();
        let rest = value % 100;
        let full_turns = value / 100;
        self.part2 += full_turns;
        
        match direction {
            'L' => self.position -= rest,
            'R' => self.position += rest,
            _ => panic!("Invalid direction"),
        }

        if self.position < 0 {
            self.position = 100 + self.position;
            if initial_position != 0 {
                self.part2 += 1;
            }
        } 
        
        if self.position > 99{
            self.position = self.position - 100;
            if self.position != 0 {
                self.part2 += 1;
            }
        }

        if self.position == 0 && rest > 0 {
            self.part1 += 1;
            self.part2 += 1;
        }
    }

    fn print_position(&self) {
        println!("Dial position: {}, cnt {}", self.position, self.part2);
    }
}

fn parse_input(input: &[String]) -> Vec<DirectionInput> {
    input.iter().map(|s| {
        let (dir, val) = s.split_at(1);
        DirectionInput {
            direction: dir.chars().next().unwrap(),
            value: val.parse().unwrap(),
        }
    }).collect()
}

fn main() {
    let input = read_input(BASE, DAY, false);

    let dir_input: Vec<DirectionInput> = parse_input(&input);
    
    let mut dial = Dial::new();
    dir_input.iter().for_each(|di| {
        dial.turn1(di.direction, di.value);
    });

    println!("{DAY} part1: {}", dial.part1);
    println!("{DAY} part2: {}", dial.part2);
}