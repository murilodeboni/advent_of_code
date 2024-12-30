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
    let mut part2: usize = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if !visited[i][j] {
                let (area, perimeter, sides) = dfs(&grid, (i,j), grid[i][j], visited, &mut 0, &mut 0, &mut 0);
                // println!("region {} is {}*{} = {}",grid[i][j],area,perimeter,area*perimeter);
                // println!("region {} is {}*{} = {}",grid[i][j],area,sides,area*sides);
                part1 += area*perimeter;
                part2 += area*sides;
            }
        }
    }

    println!("part 1 - {} , part 2 - {} | - total time - {}ms", part1, part2, start.elapsed().as_millis());
}

fn dfs(
    grid: &Vec<Vec<char>>,
    pos: (usize, usize),
    plant: char,
    visited: &mut Vec<Vec<bool>>,
    area: &mut usize,
    perimeter: &mut usize,
    sides: &mut usize
) -> (usize, usize, usize) {
    let (x, y) = pos;
    let directions: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    
    visited[x][y] = true;
    
    if grid[x][y] == plant {
        *area += 1;
    }
    let (p, s) = add_perimiter(pos, grid, &directions, plant);
    *perimeter += p;
    *sides += s;

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
                dfs(grid, (new_x, new_y), plant, visited, area, perimeter, sides);
            }
        }
    }
    return (*area,*perimeter, *sides)
}

fn add_perimiter(pos: (usize,usize), grid: &Vec<Vec<char>>, directions: &[(isize, isize); 4], plant: char) -> (usize, usize) {
    let mut perimeter: usize = 0;
    let mut sides: usize = 0; //external insight: sides = corners
    let mut dir_with_per: Vec<(isize,isize)> = Vec::new();

    for (dx, dy) in directions.iter() {
        let new_x = pos.0 as isize + dx;
        let new_y = pos.1 as isize + dy;
        if new_x < 0 {
            perimeter += 1;
            dir_with_per.push((*dx,*dy));
        } else if new_y < 0 {
            perimeter += 1;
            dir_with_per.push((*dx,*dy));
        } else {
            let new_x = new_x as usize;
            let new_y = new_y as usize;

            if new_x == grid.len() {
                perimeter += 1;
                dir_with_per.push((*dx,*dy));
            }

            if new_y == grid[0].len() {
                perimeter += 1;
                dir_with_per.push((*dx,*dy));
            }

            if (new_x) < grid.len() 
            && (new_y) < grid[0].len() {
                if grid[new_x][new_y] != plant {
                    perimeter += 1;
                    dir_with_per.push((*dx,*dy));
                }
            }
        }
    }

    if perimeter == 4 {
        sides = 4;
    } else if perimeter == 3 {
        sides = 2
    } else if perimeter == 2 {
        if (dir_with_per.contains(&(0,1))||dir_with_per.contains(&(0,-1))) && (dir_with_per.contains(&(1,0))||dir_with_per.contains(&(-1,0))) {
            sides = 1
        } else {
            sides = 0
        }
    }

    let diagonals: Vec<(isize,isize)> = vec![(1,1),(1,-1),(-1,1),(-1,-1)];
    for (dx, dy) in diagonals.iter() {
        let new_x = pos.0 as isize + dx ;
        let new_y = pos.1 as isize + dy;
        if new_x >= 0
        && new_x < grid.len() as isize 
        && new_y >= 0 
        && new_y < grid[0].len() as isize 
        && grid[pos.0][new_y as usize] == plant
        && grid[new_x as usize][pos.1] == plant 
        && grid[new_x as usize][new_y as usize] != plant {
            sides += 1
        }
    }

    (perimeter, sides)
}