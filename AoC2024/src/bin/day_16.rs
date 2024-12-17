mod utils;

use utils::input::read_lines;
use std::cmp::Ordering;

use std::{collections::{HashSet, BinaryHeap, HashMap}, time::Instant};

// Represents a position in the grid
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Position {
    i: usize, // Row index
    j: usize, // Column index
}

// Represents the direction of movement
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right, // Starting direction
}

// Represents the state in the priority queue
#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    position: Position,
    direction: Direction,
    cost: usize,           // g(n): Cost from start to current
    estimated_total: usize, // f(n) = g(n) + h(n)
}

// Implement ordering for the BinaryHeap (min-heap based on estimated_total)
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse the order for min-heap
        other
            .estimated_total
            .cmp(&self.estimated_total)
            .then_with(|| other.cost.cmp(&self.cost))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


// Calculates the Manhattan distance between two positions
fn heuristic(a: &Position, b: &Position) -> usize {
    ((a.i as isize - b.i as isize).abs() + (a.j as isize - b.j as isize).abs()) as usize
}


fn main() {
    let start = Instant::now();
    let input = read_lines("./src/bin/inputs/day_16.txt");
    let grid: Vec<Vec<char>> = input.iter().map(|l| l
        .chars()
        .collect()
    ).collect();
    let mut start = Position{i:0,j:0};
    let mut end = Position{i:0,j:0};
    
    print_grid(&grid);

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'S' {
                start = Position{i:i,j:j};
            } else if grid[i][j] == 'E' {
                end = Position{i:i,j:j};
            }
        }
    }

    let part1 = astar_shortest_path(&grid, start, end);
    
    println!("{}", part1);

}

// Reuse the previously defined structs and enums

fn astar_shortest_path(
    grid: &Vec<Vec<char>>,
    start: Position,
    end: Position,
) -> usize {
    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };

    // Directions with their (di, dj) changes
    let directions = vec![
        (Direction::Up, (-1isize, 0isize)),
        (Direction::Down, (1, 0)),
        (Direction::Left, (0, -1)),
        (Direction::Right, (0, 1)),
    ];

    // Initialize the priority queue
    let mut heap = BinaryHeap::new();

    // Initial state: position=start, direction=None, cost=0
    let initial_state = State {
        position: start.clone(),
        direction: Direction::Right,
        cost: 0,
        estimated_total: heuristic(&start, &end),
    };
    heap.push(initial_state);

    // Visited set: (i, j, direction) to handle different directions
    let mut visited: HashSet<(usize, usize, Direction)> = HashSet::new();
    visited.insert((start.i, start.j, Direction::Right));

    // Predecessors map: (i, j, direction) -> (previous Position, previous Direction)
    let mut predecessors: HashMap<(usize, usize, Direction), (Position, Direction)> =
        HashMap::new();

    while let Some(current_state) = heap.pop() {
        let current_pos = current_state.position.clone();
        let current_dir = current_state.direction;

        // Check if we have reached the end
        if current_pos == end {
            // Reconstruct the path
            return current_state.cost;
            // let mut path = Vec::new();
            // let mut key = (current_pos.i, current_pos.j, current_dir);
            // while key != (start.i, start.j, Direction::Right) {
            //     let (pos, dir) = predecessors[&key].clone();
            //     path.push(pos.clone());
            //     key = (pos.i, pos.j, dir);
            // }
            // // path.push(start.clone());
            // path.reverse();
            // return he;
        }

        // Explore neighbors
        for (dir, (di, dj)) in &directions {
            let new_i = current_pos.i as isize + di;
            let new_j = current_pos.j as isize + dj;

            // Check boundaries
            if new_i < 0 || new_j < 0 {
                continue;
            }
            let new_i = new_i as usize;
            let new_j = new_j as usize;

            if new_i >= rows || new_j >= cols {
                continue;
            }

            // Check if the cell is walkable
            if grid[new_i][new_j] == '#' {
                continue;
            }

            let neighbor_pos = Position { i: new_i, j: new_j };
            let move_dir = *dir;

            // Calculate the cost
            let mut new_cost = current_state.cost + 1; // Straight move cost

            // If direction changes, add turn cost
            if current_dir != move_dir {
                new_cost += 1000; // Turn cost
            }

            // Calculate the estimated total cost
            let est_total = new_cost + heuristic(&neighbor_pos, &end);

            // Check if this state has been visited with the same direction
            if !visited.contains(&(new_i, new_j, move_dir)) {
                visited.insert((new_i, new_j, move_dir));
                heap.push(State {
                    position: neighbor_pos.clone(),
                    direction: move_dir,
                    cost: new_cost,
                    estimated_total: est_total,
                });
                predecessors.insert((new_i, new_j, move_dir), (current_pos.clone(), current_dir));
            }
        }
    }

    // No path found
    0
}


fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        let line: String = row.iter().collect();
        println!("{}", line);
    }
}