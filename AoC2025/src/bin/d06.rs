use std::time::Instant;
use aoc_utils::read_input;

const BASE: &str = env!("CARGO_MANIFEST_DIR");
const DAY: &str = "d06";

struct Operation {
    op_sign: String,
    values: Vec<i128>,
    ans: i128,
}

impl Operation {
    fn calculate(&mut self) {
        match self.op_sign.as_str() {
            "+" => {
                self.ans = self.values.iter().sum();
            }
            "*" => {
                self.ans = self.values.iter().product();
            }
            _ => {
                panic!("unknown operation sign: {}", self.op_sign);
            }
        }
    }

    fn print(&self) {
        println!(
            "Operation: sign: {}, values: {:?}, ans: {}",
            self.op_sign, self.values, self.ans
        );
    }
}

fn transpose<T: Clone>(m: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    if m.is_empty() {
        return vec![];
    }
    let cols = m[0].len();
    (0..cols)
        .map(|c| m.iter().map(|row| row[c].clone()).collect())
        .collect()
}

fn parse_input_part1(input: Vec<String>) -> Vec<Operation> {
    let mut operations: Vec<Operation> = Vec::new();
    
    let mut operation_strings: Vec<Vec<String>> = input
        .iter()
        .map(|line| line.split_whitespace().map(str::to_string).collect())
        .collect();

    operation_strings = transpose(&operation_strings);

    operation_strings.iter().for_each(|op_strs| {
        let op_sign = op_strs.last().expect("missing sign").to_string();
        let values: Vec<i128> = op_strs[..op_strs.len() - 1]
            .iter()
            .map(|s| s.parse().expect("value should be a number"))
            .collect();
        
        operations.push(Operation { op_sign, values , ans: 0});
    });
    
    operations
}

fn parse_input_part2(input: Vec<String>) -> Vec<Operation> {
    let height = input.len();
    let width = input.iter().map(|l| l.len()).max().unwrap_or(0);

    let mut columns: Vec<Vec<char>> = Vec::with_capacity(width);
    for col in 0..width {
        let mut column_chars = Vec::with_capacity(height);
        for row in 0..height {
            let ch = input[row].chars().nth(col).unwrap_or(' ');
            column_chars.push(ch);
        }
        columns.push(column_chars);
    }

    let mut operations = Vec::new();
    let mut values_acc: Vec<i128> = Vec::new();

    // Walk columns from right-to-left; bottom char is the operator when present.
    for column in columns.into_iter().rev() {
        if column.iter().all(|c| *c == ' ') {
            continue;
        }

        let (digits_part, op_char) = column.split_at(column.len() - 1);
        let digits: String = digits_part.iter().collect();
        let digits_trimmed = digits.trim();
        if !digits_trimmed.is_empty() {
            values_acc.push(digits_trimmed.parse().expect("value should be a number"));
        }

        let op = *op_char.first().unwrap();
        if op == '+' || op == '*' {
            let op_sign = op.to_string();
            operations.push(Operation {
                op_sign,
                values: values_acc.drain(..).collect(),
                ans: 0,
            });
        }
    }

    operations
}


fn main() {
    let start = Instant::now();

    let mut part1: u128 = 0;
    let mut part2: u128 = 0;

    let input = read_input(BASE, DAY, false);

    let operations_part_1 = parse_input_part1(input.clone());
    let operations_part_2 = parse_input_part2(input);

    for mut op in operations_part_1 {
        op.calculate();
        part1 += op.ans as u128;
    }

    for mut op in operations_part_2 {
        op.calculate();
        op.print();
        part2 += op.ans as u128;
    }
    
    println!("{DAY} part1: {}", part1);
    println!("{DAY} part2: {}", part2);
    println!("Elapsed: {}us", start.elapsed().as_micros());
}
