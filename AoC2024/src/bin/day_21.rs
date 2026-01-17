// Adapted from logic described in https://observablehq.com/@jwolondon/advent-of-code-2024-day-21

mod utils;

use utils::input::read_lines;

use std::collections::HashMap;

#[derive(Debug)]
struct Pad {
    coords: HashMap<char, (usize, usize)>,
    gap: (usize,usize),
}

fn pad_lookup(pad_rows: &[&str]) -> Pad {
    let mut coords = HashMap::new();

    let gap = if pad_rows.len() == 2 {
        (0,0)
    } else {
        (3,0)
    };

    for (row, row_str) in pad_rows.iter().enumerate() {
        for (col, ch) in row_str.chars().enumerate() {
            if ch != ' ' {
                coords.insert(ch, (row, col));
            }
        }
    }

    Pad { coords, gap }
}
fn shortest_path(key1: char, key2: char, pad: &Pad) -> String {
    let (r1, c1) = pad.coords[&key1];
    let (r2, c2) = pad.coords[&key2];

    let mut up_down = String::new();
    if r2 > r1 {
        // Move down
        up_down.push_str(&"v".repeat(r2 - r1));
    } else {
        // Move up
        up_down.push_str(&"^".repeat(r1 - r2));
    }

    let mut left_right = String::new();
    if c2 > c1 {
        left_right.push_str(&">".repeat(c2 - c1));
    } else {
        left_right.push_str(&"<".repeat(c1 - c2));
    }

    if c2 > c1 && (r2, c1) != pad.gap {
        return format!("{}{}A", up_down, left_right);
    }
    if (r1, c2) != pad.gap {
        return format!("{}{}A", left_right, up_down);
    }
    format!("{}{}A", up_down, left_right)
}

fn sequences(seq: &str, pad: &Pad) -> Vec<String> {
    let mut paths = Vec::new();
    let mut prev_key = 'A';

    for ch in seq.chars() {
        let path = shortest_path(prev_key, ch, pad);
        paths.push(path);
        prev_key = ch;
    }
    paths
}

fn part1(codes: Vec<&str>) -> i32 {
    let numeric_pad = pad_lookup(&["789", "456", "123", " 0A"]);
    let dir_pad = pad_lookup(&[" ^A", "<v>"]);

    let r1_seqs: Vec<String> = codes
        .iter()
        .map(|code| sequences(code, &numeric_pad).join(""))
        .collect();

    let r2_seqs: Vec<String> = r1_seqs
        .iter()
        .map(|seq| sequences(seq, &dir_pad).join(""))
        .collect();

    let r3_seqs: Vec<String> = r2_seqs
        .iter()
        .map(|seq| sequences(seq, &dir_pad).join(""))
        .collect();

    let mut total = 0;
    for (i, seq) in r3_seqs.iter().enumerate() {
        let code = codes[i];
        if code.len() > 1 {
            let prefix = &code[..(code.len() - 1)];
            if let Ok(num) = prefix.parse::<i32>() {
                total += (seq.len() as i32) * num;
            }
        }
    }

    total
}

fn main() {
    let input = read_lines("./src/bin/inputs/day_21.txt");
    let commands: Vec<&str> = input.iter().map(|s| s.as_str()).collect();

    let part1 = part1(commands);
    println!("{}", part1)
}