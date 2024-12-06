mod utils;

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

fn direction_to_index(d: Direction) -> usize {
    match d {
        Direction::Up => 0,
        Direction::Right => 1,
        Direction::Down => 2,
        Direction::Left => 3,
    }
}

fn main() {
    let start = Instant::now();
    let input = read_lines("./src/bin/inputs/day_06.txt");
    let mut matrix: Vec<Vec<char>> = Vec::new();
    let mut start_position: Position = (0, 0);
    let initial_direction = Direction::Up;

    // Parse input and find start position
    for (row_idx, line) in input.into_iter().enumerate() {
        let parts: Vec<char> = line.chars().collect();
        if let Some(sp) = line.find('^') {
            start_position = (row_idx, sp);
        }
        matrix.push(parts);
    }

    let rows = matrix.len();
    let cols = if rows > 0 { matrix[0].len() } else { 0 };

    // Part 1: Determine visited positions
    let (positions_map, _) = go(
        initial_direction,
        start_position,
        &matrix,
        rows,
        cols,
        None,
    );
    let part_1_time = start.elapsed();

    // Part 2: Try placing an obstruction in each visited cell (except the start)
    // and see if it causes a loop
    let mut part_2_ans = 0;
    for r in 0..rows {
        for c in 0..cols {
            if positions_map[r][c] && (r, c) != start_position {
                let (_, loop_detected) = go(
                    initial_direction,
                    start_position,
                    &matrix,
                    rows,
                    cols,
                    Some((r, c)),
                );
                if loop_detected {
                    part_2_ans += 1;
                }
            }
        }
    }

    let part_2_time = start.elapsed() - part_1_time;

    // Count how many distinct positions visited in part 1
    let visited_count = positions_map.iter().flat_map(|row| row.iter()).filter(|&&b| b).count();
    println!("part 1 - {} - {}", visited_count, part_1_time.as_millis());
    println!("part 2 - {} - {}", part_2_ans, part_2_time.as_millis());
}

fn go(
    direction: Direction,
    position: Position,
    matrix: &Vec<Vec<char>>,
    rows: usize,
    cols: usize,
    extra_obstruction: Option<Position>,
) -> (Vec<Vec<bool>>, bool) {
    // visited positions
    let mut positions_map = vec![vec![false; cols]; rows];
    // visited directions: positions_direction_map[row][col][dir_idx]
    let mut positions_direction_map = vec![vec![[false; 4]; cols]; rows];

    simulate(
        direction,
        position,
        matrix,
        &mut positions_map,
        &mut positions_direction_map,
        extra_obstruction,
    )
}

fn simulate(
    direction: Direction,
    position: Position,
    matrix: &Vec<Vec<char>>,
    positions_map: &mut [Vec<bool>],
    positions_direction_map: &mut [Vec<[bool;4]>],
    extra_obstruction: Option<Position>,
) -> (Vec<Vec<bool>>, bool) {
    let (mut dir, mut pos) = (direction, position);

    loop {
        let dir_idx = direction_to_index(dir);

        // Check if we've been here with the same direction before
        if positions_direction_map[pos.0][pos.1][dir_idx] {
            // Loop detected
            return (positions_map.to_vec(), true);
        }

        // Mark current position and direction as visited
        positions_direction_map[pos.0][pos.1][dir_idx] = true;
        positions_map[pos.0][pos.1] = true;

        let next_pos = next_valid_position(pos, dir, matrix);
        // Guard leaves the area
        if next_pos.is_none() {
            return (positions_map.to_vec(), false);
        }

        let new_pos = next_pos.unwrap();

        // If blocked, turn right and try again from the same spot
        if is_blocked(new_pos, matrix, extra_obstruction) {
            dir = dir.turn();
            // do not move forward, just change direction
        } else {
            // Move forward
            pos = new_pos;
        }
    }
}

fn next_valid_position((r, c): Position, direction: Direction, matrix: &Vec<Vec<char>>) -> Option<Position> {
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

fn is_blocked(position: Position, matrix: &Vec<Vec<char>>, extra_obstruction: Option<Position>) -> bool {
    if let Some(obst) = extra_obstruction {
        if position == obst {
            return true;
        }
    }
    matrix[position.0][position.1] == '#'
}

#[allow(dead_code)]
fn print_matrix(position: Position, matrix: &Vec<Vec<char>>) {
    let mut nm = matrix.clone();
    nm[position.0][position.1] = 'K';
    for row in nm {
        println!("{}", row.iter().collect::<String>());
    }
    println!("================");
}
