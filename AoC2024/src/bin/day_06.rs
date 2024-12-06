mod utils;

use utils::input::read_lines;

use std::collections::HashMap;

fn main() {
    let input = read_lines("./src/bin/inputs/day_06.txt");

    let part_1_ans: usize;
    
    let direction: String = "up".to_string();
    
    let mut matrix:Vec<Vec<String>> = Vec::new();
    let mut position: (usize, usize) = (0,0);
    
    let mut positions_map: HashMap<(usize,usize), bool> = HashMap::new();
    
    let mut c = 0;
    for i in input {
        let parts: Vec<String> = i.chars().map(|c| c.to_string()).collect();
        if let Some(sp) = i.find("^") {
            position = (c, sp);
        }
        matrix.push(parts);
        c += 1
    }

    positions_map.insert(position, true);
    part_1_ans = go(direction, position, &matrix, positions_map);

    println!("{}", part_1_ans);
}

fn go(direction: String, position: (usize, usize), matrix: &Vec<Vec<String>>, mut positions_map: HashMap<(usize,usize), bool>) -> usize {
    // print_matrix(position, matrix);
    let new_position: (usize, usize);

    if direction == "up" {
        if position.0 >= 1 {
            new_position = (position.0 - 1, position.1);
        } else {
            print_matrix(position, matrix);
            return positions_map.len();
        }
    } else if direction == "right" {
        if position.1 < matrix[0].len() - 1 {
            new_position = (position.0 , position.1 + 1);
        } else {
            print_matrix(position, matrix);
            return positions_map.len();
        }
    } else if direction == "down" {
        if position.0 < matrix.len() - 1 {
            new_position = (position.0 + 1, position.1);
        } else {
            print_matrix(position, matrix);
            return positions_map.len();
        }
    } else {
        if position.0 >= 1 {
            new_position = (position.0, position.1 - 1);
        } else {
            print_matrix(position, matrix);
            return positions_map.len();
        }
    }
    
    let blocked = is_blocked(new_position, matrix);

    if blocked {
        return go(turn(direction), position, matrix, positions_map);
    } else {
        positions_map.insert(new_position, true);
        return go(direction, new_position, matrix, positions_map);
    }
}

fn is_blocked(position: (usize, usize), matrix: &Vec<Vec<String>>) -> bool {
    matrix[position.0][position.1] == "#"
}

fn turn(direction: String) -> String {
    if direction == "up" {
        return "right".to_string();
    } else if direction == "right" {
        return "down".to_string();
    } else if direction == "down" {
        return "left".to_string();
    } else {
        return "up".to_string();
    }
}

fn print_matrix(position: (usize, usize), matrix: &Vec<Vec<String>>) -> () {
    let mut nm = matrix.clone();
    nm[position.0][position.1] = "K".to_string();
    for m in nm {
        println!("{:?}", m.concat());
    }
    println!("================");
}