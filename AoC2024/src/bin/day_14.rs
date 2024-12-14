mod utils;

use utils::input::read_lines;

use regex::Regex;

use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct Robot {
    p: (isize, isize),
    v: (isize, isize)
}

fn main() {
    let input = read_lines("./src/bin/inputs/day_14.txt");
    let mut robots: Vec<Robot> = Vec::new();
    let r = Regex::new(r"p=(\d*),(\d*) v=(.{0,1}\d*),(.{0,1}\d*)").unwrap();
    for i in input {
        if let Some(caps) = r.captures(i.as_str()) {
            let p0: isize = caps[1].parse().expect("error");
            let p1: isize = caps[2].parse().expect("error");
            let v0: isize = caps[3].parse().expect("error");
            let v1: isize = caps[4].parse().expect("error");
            let robot = Robot { p: (p0,p1), v: (v0,v1) };
            robots.push(robot);
        }
    }
    for t in 7624.. {
        println!("Time {}", t);
        println!("{:?}", part1(&robots, t, (101,103)));
        // Sleep for 1 second
        thread::sleep(Duration::from_millis(500));

    }
}

fn part1(robots: &Vec<Robot>, seconds: isize, grid_dim: (isize, isize)) -> usize {
    let mut new_robots: Vec<Robot> = Vec::new();
    for robot in robots {
        let p0 = (robot.p.0 + seconds*robot.v.0) % grid_dim.0;
        let p1 = (robot.p.1 + seconds*robot.v.1) % grid_dim.1;
        let robot = Robot { p: (if p0 < 0 {grid_dim.0 + p0} else {p0},if p1 < 0 {grid_dim.1 + p1} else {p1}), v: robot.v };
        // println!("{:?}", robot);
        new_robots.push(robot);
    }
    print_grid(&new_robots, grid_dim);
    // calc_quadrats(&new_robots, grid_dim) -- part 1
    0
}

fn calc_quadrats(robots: &Vec<Robot>, grid_dim: (isize, isize)) -> usize {
    let (mut q1, mut q2, mut q3, mut q4) = (0, 0, 0, 0);
    for r in robots {
        if r.p.0 < (grid_dim.0-1)/2 && r.p.1 < (grid_dim.1-1)/2 {
            q1 += 1;
        } else if r.p.0 > (grid_dim.0-1)/2 && r.p.1 < (grid_dim.1-1)/2 {
            q2 += 1;
        } else if r.p.0 < (grid_dim.0-1)/2 && r.p.1 > (grid_dim.1-1)/2 {
            q3 += 1;
        } else if r.p.0 > (grid_dim.0-1)/2 && r.p.1 > (grid_dim.1-1)/2 {
            q4 += 1;
        }
    }
    println!("{} {} {} {}", q1, q2, q3, q4);
    q1*q2*q3*q4
}

fn print_grid(robots: &Vec<Robot>, grid_dim: (isize, isize)) {
    let mut grid = vec![vec![0; grid_dim.0 as usize];grid_dim.1 as usize];
    for r in robots {
        grid[r.p.1 as usize][r.p.0 as usize] += 1
    }
    for j in 0..grid.len() {
        let line: Vec<String> = grid[j].iter().map(|n| n.to_string()).collect();
        println!("{}", line.concat().replace("0", "."))
    }
}