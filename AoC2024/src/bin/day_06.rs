mod utils;

use utils::input::read_lines;

use std::collections::HashMap;

use std::time::Instant;

fn main() {
    let start = Instant::now();
    let input = read_lines("./src/bin/inputs/day_06.txt");

    let part_1_ans: usize;
    let mut part_2_ans: usize = 0;
    
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
    (positions_map, _) = go(&direction, position, &matrix, positions_map, HashMap::new());

    let part_1_time = start.elapsed();

    let final_pos_map = positions_map.clone();
    
    for (k, _) in final_pos_map {
        let mut matrix_with_obstable = matrix.clone();
        let works_for_part_2: bool;

        matrix_with_obstable[k.0][k.1] = "#".to_string();
        
        (_, works_for_part_2) = go(&direction, position, &matrix_with_obstable, HashMap::from([(position, true)]), HashMap::new());
        part_2_ans += if works_for_part_2 {1} else {0};
    }

    let part_2_time = start.elapsed() - part_1_time;

    println!("part 1 - {} - {}", positions_map.len(), part_1_time.as_secs());
    println!("part 2 - {} - {}", part_2_ans, part_2_time.as_secs());
}

fn go(direction: &String, position: (usize, usize), matrix: &Vec<Vec<String>>, mut positions_map: HashMap<(usize,usize), bool>, mut positions_direction_map: HashMap<((usize,usize), String), usize>) -> (HashMap<(usize,usize), bool>, bool) {
    // print_matrix(position, matrix);
    let new_position: (usize, usize);

    if direction == "up" {
        if position.0 >= 1 {
            new_position = (position.0 - 1, position.1);
        } else {
            return (positions_map,false);
        }
    } else if direction == "right" {
        if position.1 < matrix[0].len() - 1 {
            new_position = (position.0 , position.1 + 1);
        } else {
            return (positions_map, false);
        }
    } else if direction == "down" {
        if position.0 < matrix.len() - 1 {
            new_position = (position.0 + 1, position.1);
        } else {
            return (positions_map, false);
        }
    } else if direction == "left" {
        if position.1 >= 1 {
            new_position = (position.0, position.1 - 1);
        } else {
            return (positions_map, false);
        }
    } else {
        new_position = (0,0);
        println!("ERROR when parsin directions");
    }
    
    let blocked = is_blocked(new_position, matrix);

    if blocked {
        return go(&turn(direction), position, matrix, positions_map, positions_direction_map);
    } else {
        if let Some(value) = positions_direction_map.get(&(new_position, direction.clone())) {
            return (positions_map, true);
        } else {
            positions_direction_map.entry((new_position, direction.clone())).or_insert(1);
        }
        positions_map.insert(new_position, true);
        return go(direction, new_position, matrix, positions_map, positions_direction_map);
    }
}

fn is_blocked(position: (usize, usize), matrix: &Vec<Vec<String>>) -> bool {
    matrix[position.0][position.1] == "#"
}

fn turn(direction: &String) -> String {
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