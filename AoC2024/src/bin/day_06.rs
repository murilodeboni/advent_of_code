mod utils;

use std::collections::HashMap;
use std::time::Instant;
use utils::input::read_lines;

type Position = (usize, usize);

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn delta(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        }
    }
}

fn main() {
    let start = Instant::now();
    let input = read_lines("./src/bin/inputs/day_06.txt");
    let mut matrix: Vec<Vec<String>> = Vec::new();
    let mut start_position: Position = (0, 0);
    let initial_direction = Direction::Up;

    // Parse input and find the start position marked with '^'
    for (row_idx, line) in input.into_iter().enumerate() {
        let parts: Vec<String> = line.chars().map(|ch| ch.to_string()).collect();
        if let Some(sp) = line.find('^') {
            start_position = (row_idx, sp);
        }
        matrix.push(parts);
    }

    // Part 1: Determine visited positions and whether a loop forms
    let (positions_map, _) = go(
        initial_direction,
        start_position,
        &matrix,
        HashMap::new(),
        HashMap::new(),
    );
    let part_1_time = start.elapsed();

    // Part 2: Try placing an obstruction in each visited cell (except the start)
    // and see if it causes a loop
    let final_pos_map = positions_map.clone();
    let mut part_2_ans = 0;
    for (pos, _) in final_pos_map {
        // Don't place at the start position
        if pos == start_position {
            continue;
        }

        let mut matrix_with_obstacle = matrix.clone();
        matrix_with_obstacle[pos.0][pos.1] = "#".to_string();

        let (_, loop_detected) = go(
            initial_direction,
            start_position,
            &matrix_with_obstacle,
            HashMap::new(),
            HashMap::new(),
        );

        if loop_detected {
            part_2_ans += 1;
        }
    }

    let part_2_time = start.elapsed() - part_1_time;

    println!("part 1 - {} - {}", positions_map.len(), part_1_time.as_secs());
    println!("part 2 - {} - {}", part_2_ans, part_2_time.as_secs());
}

fn go(
    direction: Direction,
    position: Position,
    matrix: &Vec<Vec<String>>,
    mut positions_map: HashMap<Position, bool>,
    mut positions_direction_map: HashMap<(Position, Direction), usize>,
) -> (HashMap<Position, bool>, bool) {
    // Check if we've been here with the same direction before
    if positions_direction_map.contains_key(&(position, direction)) {
        // Loop detected
        return (positions_map, true);
    }

    // Record current position and direction
    positions_direction_map.insert((position, direction), 1);
    positions_map.insert(position, true);

    let new_position = match next_valid_position(position, direction, matrix) {
        Some(pos) => pos,
        None => return (positions_map, false), // No valid move: guard leaves the area
    };

    // If blocked, turn right and try again from the same spot
    if is_blocked(new_position, matrix) {
        return go(direction.turn(), position, matrix, positions_map, positions_direction_map);
    }

    // Move forward
    go(direction, new_position, matrix, positions_map, positions_direction_map)
}

fn next_valid_position(
    (r, c): Position,
    direction: Direction,
    matrix: &Vec<Vec<String>>,
) -> Option<Position> {
    let (dr, dc) = direction.delta();
    let new_r = r as isize + dr;
    let new_c = c as isize + dc;

    if new_r < 0
        || new_r >= matrix.len() as isize
        || new_c < 0
        || new_c >= matrix[0].len() as isize
    {
        None
    } else {
        Some((new_r as usize, new_c as usize))
    }
}

fn is_blocked(position: Position, matrix: &Vec<Vec<String>>) -> bool {
    matrix[position.0][position.1] == "#"
}

#[allow(dead_code)]
fn print_matrix(position: Position, matrix: &Vec<Vec<String>>) {
    let mut nm = matrix.clone();
    nm[position.0][position.1] = "K".to_string();
    for row in nm {
        println!("{}", row.concat());
    }
    println!("================");
}
