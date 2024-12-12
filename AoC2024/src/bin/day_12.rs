mod utils;

use utils::input::read_lines;

use std::time::Instant;

fn main() {
    let start = Instant::now();
    let input = read_lines("./src/bin/inputs/day_12.txt");
    let grid: Vec<Vec<char>> = input.iter().map(|l| l
        .chars()
        .collect()
    ).collect();

    let visited: &mut Vec<Vec<bool>> = &mut vec![vec![false; grid[0].len()]; grid.len()];

    let mut part1: usize = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if !visited[i][j] {
                let (area, perimeter) = dfs(&grid, (i,j), grid[i][j], visited, &mut 0, &mut 0);
                // println!("region {} is {}*{} = {}",grid[i][j],area,perimeter,area*perimeter);
                part1 += area*perimeter;
            }
        }
    }

    println!("part 1 - {} | - total time - {}ms", part1, start.elapsed().as_millis());
}

fn dfs(
    grid: &Vec<Vec<char>>,
    pos: (usize, usize),
    plant: char,
    visited: &mut Vec<Vec<bool>>,
    area: &mut usize,
    perimeter: &mut usize
) -> (usize, usize) {
    let (x, y) = pos;
    let directions: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    
    visited[x][y] = true;
    
    if grid[x][y] == plant {
        *area += 1;
    }

    *perimeter += add_perimiter(pos, grid, &directions, plant);

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

            if !visited[new_x][new_y] && grid[new_x][new_y] == plant {
                dfs(grid, (new_x, new_y), plant, visited, area, perimeter);
            }
        }
    }
    return (*area,*perimeter)
}

fn add_perimiter(pos: (usize,usize), grid: &Vec<Vec<char>>, directions: &[(isize, isize); 4], plant: char) -> usize {
    let mut perimeter: usize = 0;
    for (dx, dy) in directions.iter() {
        let new_x = pos.0 as isize + dx;
        let new_y = pos.1 as isize + dy;
        if new_x < 0 {
            perimeter += 1
        } else if new_y < 0 {
            perimeter += 1
        } else {
            let new_x = new_x as usize;
            let new_y = new_y as usize;

            if new_x == grid.len() {
                perimeter += 1;
            }

            if new_y == grid[0].len() {
                perimeter += 1
            }

            if (new_x) < grid.len() 
            && (new_y) < grid[0].len() {
                if grid[new_x][new_y] != plant {
                    perimeter += 1
                }
            }
        }
    }
    perimeter
}

fn print_grid(grid: &Vec<Vec<u32>>) {
    for j in 0..grid.len() {
        println!("{:?}", grid[j])
    }
}