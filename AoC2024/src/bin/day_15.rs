mod utils;

use utils::input::read_lines;

use std::{collections::HashMap, time::Instant};

fn main() {
    let start = Instant::now();
    let grid_input: Vec<String> = read_lines("./src/bin/inputs/day_15_1_test.txt");
    let commands_input: Vec<String> = read_lines("./src/bin/inputs/day_15_2_test.txt");
    // let commands_input: Vec<String> = vec![">>>vvvvv".to_string()]; // TESTING
    let grid_collect_part_1: Vec<Vec<char>> = grid_input
        .iter()
        .map(|line| line.chars().collect())
        .collect();

    fn replacements(s: &String) -> String {
        s.replace("#", "##").replace(".", "..").replace("@", "@.").replace("O", "[]")
    }

    let grid_collect_part_2: Vec<Vec<char>> = grid_input
        .iter()
        .map(|line| replacements(line).chars().collect())
        .collect();

    let mut grid: Vec<Vec<char>> = grid_collect_part_2;

    print_grid(&grid);

    let mut initial_position: (usize,usize) = (0,0);
    let direction_map: HashMap<char, (isize, isize)> = HashMap::from([
        ('>', (0, 1)),
        ('v', (1, 0)),
        ('^', (-1, 0)),
        ('<', (0, -1)),
    ]);
    
    let mut part1: usize = 0;
    let mut part2: usize = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '@' {
                initial_position = (i,j);
            }
        }
    }

    for line in commands_input {
        for command in line.chars() {
            // println!("Processing command: '{}'", command);
            if direction_map.contains_key(&command) {
                let success = move_position(&mut grid, &mut initial_position, command, &direction_map);
                // if success {
                //     println!("Moved '{}' successfully.", command);
                // } else {
                //     println!("Failed to move '{}' due to obstacle.", command);
                // }
                // print_grid(&grid);
            } else {
                // println!("Unknown command: '{}'", command);
            }
        }
    }
    
    print_grid(&grid);
    // println!("{:?}", initial_position);

    let part1 = box_coordinates_sum(&grid);

    println!("part 1 - {} total time - {}ms", part1, start.elapsed().as_millis());
}

fn move_position(
    grid: &mut Vec<Vec<char>>,
    position: &mut (usize, usize),
    direction: char,
    direction_map: &HashMap<char, (isize, isize)>,
) -> bool {
    // Get movement offsets
    let to = direction_map.get(&direction).unwrap();

    // Calculate new position with boundary checks
    let new_i = (position.0 as isize + to.0) as usize;
    let new_j = (position.1 as isize + to.1) as usize;

    // Ensure new_i and new_j are within grid bounds
    if new_i >= grid.len()-1 || new_j >= grid[0].len()-1 || new_i <= 0 || new_j <= 0 {
        // println!("Move into wall: ({}, {})", new_i, new_j);
        return false;
    }

    let target_cell = grid[new_i][new_j];

    // Current cell content
    let original = grid[position.0][position.1];

    if target_cell == '.' {
        // println!("Moving into empty space at ({}, {}).", new_i, new_j);
        grid[position.0][position.1] = '.'; // Clear original position
        grid[new_i][new_j] = original; // Move player to new position
        *position = (new_i, new_j); // Update player's position
        true
    } else if target_cell == 'O' {
        // println!("Encountered rock at ({}, {}). Attempting to push.", new_i, new_j);
        // Attempt to push the rock in the same direction
        let can_push = move_position(grid, &mut (new_i, new_j), direction, direction_map);
        if can_push {
            // After pushing the rock, move the player
            grid[position.0][position.1] = '.'; // Clear original position
            grid[new_i][new_j] = original; // Move player to new position
            *position = (new_i, new_j); // Update player's position
            true
        } else {
            // println!("Failed to push the rock at ({}, {}).", new_i, new_j);
            false
        }
    } else {
        // println!(
        //     // "Cannot move to ({}, {}): Cell occupied by '{}'.",
        //     new_i, new_j, target_cell
        // );
        false
    }
}

fn box_coordinates_sum(grid: &Vec<Vec<char>>) -> usize {
    let mut ans = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'O' {
                ans += 100*i + j;
            }
        }
    }
    ans
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        let line: String = row.iter().collect();
        println!("{}", line);
    }
}