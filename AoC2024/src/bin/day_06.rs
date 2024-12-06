mod utils;

use utils::input::read_lines;

use std::{collections::HashMap, hash::Hash};

fn main() {
    let input = read_lines("./src/bin/inputs/day_06_test.txt");

    let mut part1Ans: usize = 0;
    let mut part2Ans: usize = 0;

    let mut direction: usize = 0;
    let mut go_map: HashMap<usize,(i8,i8)> = HashMap::new();
    go_map.insert(0, (-1,0));
    go_map.insert(1, (0,1));
    go_map.insert(2, (1,0));
    go_map.insert(3, (0,-1));

    let mut matrix:Vec<Vec<String>> = Vec::new();
    let mut position: (usize, usize) = (0,0);
    
    let mut c = 0;
    for i in input {
        let parts: Vec<String> = i.chars().map(|c| c.to_string()).collect();
        if let Some(sp) = i.find("^") {
            position = (c, sp);
        }
        matrix.push(parts);
        c += 1
    }

    part1Ans += go(direction, go_map, &matrix, position);

    println!("{}", part1Ans);
    println!("{}", part2Ans);

}

fn go(mut direction: usize, go_map: HashMap<usize,(i8,i8)>, matrix: &Vec<Vec<String>>, mut position: (usize, usize)) -> usize {
    let max_y = matrix.len();
    let max_x = matrix[0].len();

    let mut positions_map: HashMap<(usize,usize), usize> = HashMap::new();

    while true {
        positions_map.insert(position, 1);
        if let Some(to) = go_map.get(&direction) {
            let np = (position.0 as isize + to.0 as isize, position.1 as isize + to.1 as isize);
            if np.0 >= 0 && np.1 >= 0 && np.0 < max_x as isize && np.1 < max_y as isize {
                let np = (np.0 as usize, np.1 as usize);
                let nd = is_blocked(&matrix, np, direction);
                if nd == direction {
                    position = np;
                }
            } else {
                return positions_map.len()
            }
        }
    }

    positions_map.len()
}

fn is_blocked(matrix: &Vec<Vec<String>>, position: (usize, usize), direction: usize) -> usize {
    let mut nm = matrix.clone();
    nm[position.0][position.1] = "K".to_string();
    
    println!("=============");

    if matrix[position.0][position.1] == "#" {
        turn(direction)
    } else {
        direction
    }
}

fn turn(direction: usize) -> usize {
    if direction == 3 {
        0
    } else {
        direction + 1
    }
}
