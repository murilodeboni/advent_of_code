mod utils;

use std::collections::VecDeque;

use utils::input::read_lines;

use std::time::Instant;


fn main() {
    let start_time = Instant::now();
    let input = read_lines("./src/bin/inputs/day_18.txt");
    let grid_size: usize = 70;
    let bytes_simulated = 1024;

    let mut coordinate: Vec<(usize, usize)> = Vec::new();
    let mut grid = vec![vec!["."; grid_size+1];grid_size+1];
    let mut current_space: (usize, usize) = (0,0);
    let end_space = (grid_size, grid_size);

    for i in input {
        let parts: Vec<&str> = i.split(",").collect();
        
        let p1: usize = parts[0].parse().expect("error");
        let p2: usize = parts[1].parse().expect("error");

        coordinate.push((p1,p2));
    }
    
    let mut halt = false;
    let mut i = 0;
    while !halt {
        let (x,y) = coordinate[i];
        grid[y][x] = "#";
        
        if i >= bytes_simulated {
            if let Some(part1) = bfs(&grid, current_space, end_space) {
                if i == bytes_simulated {
                    println!("part 1 - {} took {}ms", part1, start_time.elapsed().as_millis());
                }
            } else {
                println!("can't find path after coord {:?} in {} bytes fallen in {} ms", coordinate[i], i, start_time.elapsed().as_millis());
                halt = true;
            }
        }
        i += 1;
    }
}

fn bfs(grid: &Vec<Vec<&str>>, start: (usize, usize), end: (usize, usize)) -> Option<usize> {
    let rows = grid.len();
    let cols = grid[0].len();

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut visited = vec![vec![false; cols]; rows];

    let mut queue = VecDeque::new();

    // Start BFS
    queue.push_back((start, 0));
    visited[start.0][start.1] = true;

    while let Some(((x, y), dist)) = queue.pop_front() {
        if (x, y) == end {
            return Some(dist);
        }

        // Explore neighbors
        for (dx, dy) in &directions {
            let new_x = x as isize + dx;
            let new_y = y as isize + dy;

            // Check if the neighbor is within bounds
            if new_x >= 0
                && new_y >= 0
                && new_x < rows as isize
                && new_y < cols as isize
            {
                let (new_x, new_y) = (new_x as usize, new_y as usize);

                // Check if the cell is free and not visited
                if grid[new_x][new_y] == "." && !visited[new_x][new_y] {
                    visited[new_x][new_y] = true;
                    queue.push_back(((new_x, new_y), dist + 1));
                }
            }
        }
    }
    None
}


fn print_grid(grid: &Vec<Vec<&str>>) {
    for j in 0..grid.len() {
        println!("{}", grid[j].concat())
    }
}