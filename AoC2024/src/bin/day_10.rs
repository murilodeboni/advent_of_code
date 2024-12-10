mod utils;

use utils::input::read_lines;

use std::collections::{HashMap, HashSet};

fn main() {
    let input = read_lines("./src/bin/inputs/day_10.txt");
    let grid: Vec<Vec<u32>> = input.iter().map(|l| l
        .chars()
        .map(|c| c.
            to_digit(10).
            expect("error")
        ).collect()
    ).collect();

    // print_grid(&grid);

    let mut part1: usize = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 0 {
                // println!("({},{})",i,j);
                let score = dfs(&grid, (i,j), &mut vec![vec![false; grid[0].len()]; grid.len()], &mut 0);
                part1 += score;
            }
        }
    }

    println!("{}", part1);
}

fn dfs(
    grid: &Vec<Vec<u32>>,
    pos: (usize, usize),
    visited: &mut Vec<Vec<bool>>,
    count: &mut usize,
) -> usize {
    let (x, y) = pos;
    visited[x][y] = true;

    if grid[x][y] == 9 {
        *count += 1;
    }

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

            if !visited[new_x][new_y] && grid[new_x][new_y] == grid[x][y] + 1 {
                dfs(grid, (new_x, new_y), visited, count);
            }
        }
    }
    return *count
}

fn print_grid(grid: &Vec<Vec<u32>>) {
    for j in 0..grid.len() {
        println!("{:?}", grid[j])
    }
}