mod utils;

use utils::input::read_lines;

use std::{collections::{HashMap, HashSet, VecDeque}, time::Instant};

fn main() {
    let start = Instant::now();
    let grid_input: Vec<String> = read_lines("./src/bin/inputs/day_15_1_test.txt");
    let commands_input: Vec<String> = read_lines("./src/bin/inputs/day_15_2_test.txt");
    let commands: Vec<char> = commands_input.iter().flat_map(|line| line.chars()).collect();

    let mut grid_part_1: Vec<Vec<char>> = grid_input
        .iter()
        .map(|line| line.chars().collect())
        .collect();

    fn replacements(s: &String) -> String {
        s.replace("#", "##").replace(".", "..").replace("@", "@.").replace("O", "[]")
    }

    let mut grid_part_2: Vec<Vec<char>> = grid_input
        .iter()
        .map(|line| replacements(line).chars().collect())
        .collect();

    let part1 = simulate_part1(&mut grid_part_1, &commands);
    let part2 = simulate_part2(&mut grid_part_2, &commands);

    println!("part 1 - {}", part1);
    println!("part 2 - {} total time - {}ms", part2, start.elapsed().as_millis());
}

fn simulate_part1(grid: &mut Vec<Vec<char>>, commands: &[char]) -> usize {
    let mut robot = find_robot(grid);
    let direction_map: HashMap<char, (isize, isize)> = HashMap::from([
        ('>', (0, 1)),
        ('v', (1, 0)),
        ('^', (-1, 0)),
        ('<', (0, -1)),
    ]);

    for &command in commands {
        if let Some(&dir) = direction_map.get(&command) {
            move_part1(grid, &mut robot, dir);
        }
    }

    gps_sum(grid, 'O')
}

fn simulate_part2(grid: &mut Vec<Vec<char>>, commands: &[char]) -> usize {
    let mut robot = find_robot(grid);
    let direction_map: HashMap<char, (isize, isize)> = HashMap::from([
        ('>', (0, 1)),
        ('v', (1, 0)),
        ('^', (-1, 0)),
        ('<', (0, -1)),
    ]);

    for &command in commands {
        if let Some(&dir) = direction_map.get(&command) {
            move_part2(grid, &mut robot, dir);
        }
    }

    gps_sum(grid, '[')
}

fn move_part1(grid: &mut Vec<Vec<char>>, position: &mut (usize, usize), direction: (isize, isize)) -> bool {
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;
    let mut target_i = position.0 as isize + direction.0;
    let mut target_j = position.1 as isize + direction.1;

    while target_i >= 0 && target_i < rows && target_j >= 0 && target_j < cols {
        match grid[target_i as usize][target_j as usize] {
            '#' => return false,
            '.' => break,
            _ => {
                target_i += direction.0;
                target_j += direction.1;
            }
        }
    }

    if target_i < 0 || target_i >= rows || target_j < 0 || target_j >= cols {
        return false;
    }

    let mut cur_i = target_i;
    let mut cur_j = target_j;
    while cur_i != position.0 as isize || cur_j != position.1 as isize {
        let prev_i = cur_i - direction.0;
        let prev_j = cur_j - direction.1;
        grid[cur_i as usize][cur_j as usize] = grid[prev_i as usize][prev_j as usize];
        cur_i = prev_i;
        cur_j = prev_j;
    }

    grid[position.0][position.1] = '.';
    position.0 = (position.0 as isize + direction.0) as usize;
    position.1 = (position.1 as isize + direction.1) as usize;
    grid[position.0][position.1] = '@';
    true
}

fn move_part2(grid: &mut Vec<Vec<char>>, position: &mut (usize, usize), direction: (isize, isize)) -> bool {
    if direction.0 == 0 {
        return move_part1(grid, position, direction);
    }

    let start_i = position.0 as isize + direction.0;
    let start_j = position.1 as isize + direction.1;
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;

    if start_i < 0 || start_i >= rows || start_j < 0 || start_j >= cols {
        return false;
    }

    let mut queue = VecDeque::new();
    let mut boxes = Vec::new();
    let mut seen = HashSet::new();

    queue.push_back((start_i as usize, start_j as usize));

    while let Some((i, j)) = queue.pop_front() {
        match grid[i][j] {
            '.' => continue,
            '#' => return false,
            '[' => {
                if seen.insert((i, j)) {
                    boxes.push((i, j));
                    let next_i = i as isize + direction.0;
                    if next_i < 0 || next_i >= rows {
                        return false;
                    }
                    queue.push_back((next_i as usize, j));
                    queue.push_back((next_i as usize, j + 1));
                }
            }
            ']' => {
                let left = j - 1;
                if seen.insert((i, left)) {
                    boxes.push((i, left));
                    let next_i = i as isize + direction.0;
                    if next_i < 0 || next_i >= rows {
                        return false;
                    }
                    queue.push_back((next_i as usize, left));
                    queue.push_back((next_i as usize, left + 1));
                }
            }
            _ => {}
        }
    }

    for &(i, j) in &boxes {
        grid[i][j] = '.';
        grid[i][j + 1] = '.';
    }

    for &(i, j) in &boxes {
        let next_i = (i as isize + direction.0) as usize;
        grid[next_i][j] = '[';
        grid[next_i][j + 1] = ']';
    }

    grid[position.0][position.1] = '.';
    position.0 = (position.0 as isize + direction.0) as usize;
    grid[position.0][position.1] = '@';
    true
}

fn find_robot(grid: &Vec<Vec<char>>) -> (usize, usize) {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '@' {
                return (i, j);
            }
        }
    }
    (0, 0)
}

fn gps_sum(grid: &Vec<Vec<char>>, target: char) -> usize {
    let mut ans = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == target {
                ans += 100 * i + j;
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
