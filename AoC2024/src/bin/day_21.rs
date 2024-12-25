mod utils;

use std::{collections::{HashMap, VecDeque}, hash::Hash};

use utils::input::read_lines;

struct Pad {
    grid: Vec<Vec<char>>,
    curr: (usize,usize)
}

impl Pad {
    fn initNumPad(&mut self) {
        self.grid = vec![
            vec!['7','8','9'],
            vec!['4','5','6'],
            vec!['1','2','3'],
            vec!['#','0','A']
        ];
        self.curr = (3,2);
    }
    fn initDirPad(&mut self) {
        self.grid = vec![
            vec!['#','^','A'],
            vec!['<','v','>']
        ];
        self.curr = (0,2);
    }

    fn parse_command(&mut self, command: Vec<char>) -> Vec<char> {
        let mut ans: Vec<char> = Vec::new();
        for c in command {
            let mut path = self.goto(c);
            self.reaplce_curr(c);
            ans.append(&mut path);
        }
        ans
    }

    fn goto(&mut self, end: char) -> Vec<char> {
        let mut ans = bfs(&self.grid, self.curr, end);
        ans.push('A');
        return ans
    }

    fn reaplce_curr(&mut self, c: char) {
        for i in 0..self.grid.len() {
            for j in 0..self.grid[0].len() {
                if self.grid[i][j] == c {
                    self.curr = (i,j);
                }
            }
        }
    }
}

fn main() {
    let input = read_lines("./src/bin/inputs/day_21_test.txt");
    let commands: Vec<Vec<char>> = input.iter().map(|s| s.chars().collect()).collect();

    let mut part1 = 0;

    let mut robot1Num = Pad{grid: Vec::new(), curr: (0,0)};
    robot1Num.initNumPad();

    let mut robot1Dir = Pad{grid: Vec::new(), curr: (0,0)};
    robot1Dir.initDirPad();
    
    let mut robot2Dir = Pad{grid: Vec::new(), curr: (0,0)};
    robot2Dir.initDirPad();

    for command in commands {
        let l1 = robot1Num.parse_command(command.clone());
        let l2 = robot1Dir.parse_command(l1);
        let l3 = robot1Dir.parse_command(l2);
        println!("{} {} {}", vec_char_to_str(command), l3.len(), vec_char_to_str(l3));
    }

    println!("{}", part1)

}

fn bfs(grid: &Vec<Vec<char>>, start: (usize, usize), end: char) -> Vec<char> {
    let rows = grid.len();
    let cols = grid[0].len();

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let dirMap: HashMap<(isize, isize), char> = HashMap::from([
        ((0, 1),'>'),
        ((1, 0),'^'),
        ((0, -1),'<'),
        ((-1, 0),'v')
    ]);
    let mut visited = vec![vec![false; cols]; rows];

    let mut queue = VecDeque::new();

    // Start BFS
    queue.push_back((start, Vec::new()));
    visited[start.0][start.1] = true;

    while let Some(((x, y), path)) = queue.pop_front() {
        if grid[x][y] == end {
            return path;
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
                if grid[new_x][new_y] != '#' && !visited[new_x][new_y] {
                    visited[new_x][new_y] = true;
                    let mut new_path = path.clone();
                    new_path.push(*dirMap.get(&(*dx,*dy)).unwrap());
                    queue.push_back(((new_x, new_y), new_path));
                }
            }
        }
    }
    Vec::new()
}


fn vec_char_to_str(v: Vec<char>) -> String {
    let vs:Vec<String> = v.iter().map(|c|c.to_string()).collect();
    return vs.concat()
}

fn print_grid(grid: &Vec<Vec<&str>>) {
    for j in 0..grid.len() {
        println!("{}", grid[j].concat())
    }
}