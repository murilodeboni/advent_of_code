mod utils;

use utils::input::read_lines;

use std::{collections::HashMap, time::Instant};

#[derive(Eq, PartialEq, Clone, Debug)]
struct Cheat {
    x: usize,
    y: usize,
    dir: (isize,isize)
}

fn main() {
    let start = Instant::now();
    let grid_input: Vec<String> = read_lines("./src/bin/inputs/day_20_test.txt");
    let grid: Vec<Vec<char>> = grid_input
        .iter()
        .map(|line| line.chars().collect())
        .collect();

    print_grid(&grid);
    let mut cheats: Vec<Cheat> = Vec::new();
    let mut start: (usize,usize) = (0,0);
    let mut end: (usize,usize) = (0,0);
    
    let mut part1: usize = 0;
    
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'S' {
                start = (i,j);
            }
            if grid[i][j] == 'E' {
                end = (i,j);
            }
        }
    }

    let (count, costs_without_cheat) = dfs_no_cheat(&grid, start, &mut vec![vec![false; grid[0].len()]; grid.len()], &mut HashMap::new(), &mut 0);
    println!("{} {:?}",count, costs_without_cheat)

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

fn dfs(
    grid: &Vec<Vec<char>>,
    pos: (usize, usize),
    visited: &mut Vec<Vec<bool>>,
    has_cheated: Option<Cheat>,
    cheated: &mut Vec<Cheat>,
    count: &mut usize,
) -> (usize, Option<Cheat>) {

    let (x, y) = pos;
    visited[x][y] = true;

    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let cheat_directions = [(0, 2), (0, -2), (2, 0), (-2, 0)];

    // if has_cheated.is_none() {
    //     for &(dx, dy) in cheat_directions.iter() {
    //         let new_x = x as isize + dx;
    //         let new_y = y as isize + dy;

    //         if new_x >= 0
    //             && new_y >= 0
    //             && (new_x as usize) < grid.len()
    //             && (new_y as usize) < grid[0].len()
    //         {
    //             let new_x = new_x as usize;
    //             let new_y = new_y as usize;

    //             let new_x_half = if dx > 0 {new_x-1} else if dx < 0 {new_x+1} else {new_x};
    //             let new_y_half = if dy > 0 {new_y-1} else if dy < 0 {new_y+1} else {new_y};

    //             if !visited[new_x][new_y] && has_cheated.is_none() && grid[new_x_half][new_y_half] == '#' && !cheated.contains(&Cheat{x: new_x, y: new_y, dir: (dx,dy)}) && grid[new_x][new_y] == '.' {
    //                 println!("cheated jumping from {:?} to {:?} above the wall {:?}", (x,y), (new_x,new_y), (new_x_half, new_y_half));
    //                 let cheat = Cheat{x: new_x, y: new_y, dir: (dx,dy)};
    //                 cheated.push(cheat.clone());
    //                 *count += 2;
    //                 dfs(grid, (new_x, new_y), visited, Some(cheat), cheated, count);
    //             }
    //         }
    //     }
    // } else {
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
                    dfs(grid, (new_x, new_y), visited, has_cheated.clone(), cheated, count);
                }
            }
        }
    // }
    return (*count, has_cheated);
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        let line: String = row.iter().collect();
        println!("{}", line);
    }
}