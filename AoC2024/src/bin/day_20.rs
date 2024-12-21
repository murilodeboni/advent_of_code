mod utils;

use utils::input::read_lines;

use std::{collections::HashMap, time::Instant};

fn main() {
    let start_time = Instant::now();
    let grid_input: Vec<String> = read_lines("./src/bin/inputs/day_20.txt");
    let grid: Vec<Vec<char>> = grid_input
        .iter()
        .map(|line| line.chars().collect())
        .collect();

    let mut start: (usize,usize) = (0,0);
    
    let mut part1: usize = 0;
    let mut part2: usize = 0;
    
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'S' {
                start = (i,j);
            }
        }
    }


    let (_,mut costs_without_cheat) = dfs_no_cheat(&grid, start, &mut vec![vec![false; grid[0].len()]; grid.len()], &mut HashMap::new(), &mut 0);
    // println!("{} {:?}",count, costs_without_cheat);
    costs_without_cheat.insert(start,0);

    // seconds saved, cheat count
    let mut cheat_savings_part1: HashMap<usize, usize> = HashMap::new();
    let mut cheat_savings_part2: HashMap<usize, usize> = HashMap::new();

    let cheat_directions = [(0, 2), (0, -2), (2, 0), (-2, 0)];
    for (pos, _) in &costs_without_cheat {
        for dir in cheat_directions {
            if let Some(savings) = cheat(&grid, &costs_without_cheat, pos.0, pos.1, dir.0, dir.1) {
                *cheat_savings_part1.entry(savings).or_insert(0) += 1
            }
        }
    }

    for (savings, count) in &cheat_savings_part1 {
        if savings >= &100 {
            part1 += count;
        }
    }
    println!("part 1 - {} took {}", part1, start_time.elapsed().as_millis());

    for ((x1, y1), c1) in &costs_without_cheat {
        for ((x2, y2), c2) in &costs_without_cheat {
            let cheat_cost = x1.abs_diff(*x2) + y1.abs_diff(*y2);
            let cheat_saving: isize = *c2 as isize - *c1 as isize - cheat_cost as isize;
            if cheat_saving > 0 && cheat_cost <= 20 {
                *cheat_savings_part2.entry(cheat_saving as usize).or_insert(0) += 1;
            }
        }
    }

    for (savings, count) in &cheat_savings_part2 {
        if savings >= &100 {
            part2 += count;
        }
    }
    println!("part 2 - {} took {}", part2, start_time.elapsed().as_millis());


}

fn dfs_no_cheat(
    grid: &Vec<Vec<char>>,
    pos: (usize, usize),
    visited: &mut Vec<Vec<bool>>,
    cost:&mut  HashMap<(usize,usize), usize>,
    count: &mut usize,
) -> (usize, HashMap<(usize,usize), usize>) {

    let (x, y) = pos;
    visited[x][y] = true;

    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    for &(dx, dy) in directions.iter() {
        let new_x = x as isize + dx;
        let new_y = y as isize + dy;

        if new_x >= 0
            && new_y >= 0
            && (new_x as usize) < grid.len()
            && (new_y as usize) < grid[0].len()
        {
            let new_x = new_x as usize;
            let new_y = new_y as usize;

            if !visited[new_x][new_y] && (grid[new_x][new_y] == '.'|| grid[new_x][new_y] == 'E') {
                *count += 1;
                cost.insert((new_x,new_y), *count);
                dfs_no_cheat(grid, (new_x, new_y), visited, cost, count);
            }
        }
    }
    return (*count, cost.clone());
}

fn cheat(
    grid: &Vec<Vec<char>>, cost: &HashMap<(usize,usize), usize>, x: usize, y: usize, dx: isize, dy: isize
) -> Option<usize> {
    let new_x = x as isize + dx;
    let new_y = y as isize + dy;

    if new_x >= 0
        && new_y >= 0
        && (new_x as usize) < grid.len()
        && (new_y as usize) < grid[0].len()
    {
        let new_x = new_x as usize;
        let new_y = new_y as usize;

        let new_x_half = if dx > 0 {new_x-1} else if dx < 0 {new_x+1} else {new_x};
        let new_y_half = if dy > 0 {new_y-1} else if dy < 0 {new_y+1} else {new_y};

        let starting_cost = cost.get(&(x,y)).unwrap();

        if grid[new_x_half][new_y_half] == '#' && (grid[new_x][new_y] == '.' || grid[new_x][new_y] == 'E') {
            if let Some(new_cost) = cost.get(&(new_x, new_y)) {
                // println!("cheated jumping from {:?} ({}) to {:?}({}) above the wall {:?}", (x,y), starting_cost, (new_x,new_y), new_cost, (new_x_half, new_y_half));
                if new_cost >= &(starting_cost + 2) {
                    return Some(new_cost.saturating_sub(starting_cost + 2))
                }
            } else {
                return None
            }
        }
    }

    None
}

#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        let line: String = row.iter().collect();
        println!("{}", line);
    }
}
