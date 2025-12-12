use std::collections::{HashMap, HashSet, VecDeque};
use std::time::Instant;
use aoc_utils::read_input;

const BASE: &str = env!("CARGO_MANIFEST_DIR");
const DAY: &str = "d09";

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum Color {
    Red,
    Green
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: isize,
    y: isize
}

struct Rectangle {
    c1: Point,
    c2: Point,
}

impl Rectangle {
    fn area(&self) -> isize {
        let width = (self.c2.x - self.c1.x+1).abs();
        let height = (self.c2.y - self.c1.y+1).abs();
        width * height
    }
}

fn parse_input(input: Vec<String>) -> Vec<Point> {
    let mut output: Vec<Point> = Vec::<Point>::new();
    input.iter().for_each(|line| {
        let ls: Vec<&str> = line.split(",").collect();
        let x: isize = ls[0].parse().unwrap();
        let y: isize = ls[1].parse().unwrap();
        output.push(Point { x, y });
    });
    output
}

fn get_green_perimeter(red_points: &Vec<Point>) -> Vec<Point> {
    let mut green_points: Vec<Point> = Vec::<Point>::new();    
    let mut cp: &Point = red_points.first().unwrap();
    for rp in red_points[1..].iter() {
        if cp.x == rp.x {
            let y_start = cp.y.min(rp.y);
            let y_end = cp.y.max(rp.y);
            for y in (y_start+1)..y_end {
                green_points.push(Point { x: cp.x, y});
            }
        } else if cp.y == rp.y {
            let x_start = cp.x.min(rp.x);
            let x_end = cp.x.max(rp.x);
            for x in (x_start+1)..x_end {
                green_points.push(Point { x, y: cp.y});
            }
        }
    }
    green_points
}

fn is_area_possible(rect: &Rectangle, hash: &HashSet<Point>) -> bool {
    let mut is_possible: bool = true;
    
    let (c1, c2) = (&rect.c1, &rect.c2);
    let (c3, c4) = (
        &Point { x: c1.x, y: c2.y},
        &Point { x: c2.x, y: c1.y},
    );

    let perimeter_points = vec![c1, c2, c3, c4];
    for p in perimeter_points.iter() {
        if !hash.contains(p) {
            is_possible = false;
            break;
        }
    }
    return is_possible;
    
}



fn find_largest_rectangle(
    red_points: &Vec<Point>
    , hash: &HashSet<Point>
) -> (isize, isize) {
    let mut largest_area_1: isize = 0;
    let mut largest_area_2: isize = 0;
     
    for p1 in red_points.iter() {
        for p2 in red_points.iter() {
            if p1 != p2 {
                let rect = Rectangle {
                    c1: Point { x: p1.x.min(p2.x), y: p1.y.min(p2.y)},
                    c2: Point { x: p1.x.max(p2.x), y: p1.y.max(p2.y)},
                };
                let area = rect.area();
                if area > largest_area_1 {
                    largest_area_1 = area;
                }
                if area > largest_area_2 && is_area_possible(&rect, hash) {
                    println!("Found possible rectangle with dimensions: ({}, {}) to ({}, {}) with area {}", rect.c1.x, rect.c1.y, rect.c2.x, rect.c2.y, area);
                    largest_area_2 = area;
                }
            }
        }
    }
    (largest_area_1, largest_area_2)
}

fn main() {
    let start = Instant::now();

    let input = read_input(BASE, DAY, true);

    let _red_points = parse_input(input);

    let _green_points: Vec<Point> = get_green_perimeter(&_red_points);

    let mut hash: HashSet<Point> = HashSet::<Point>::new();
    _red_points.iter().chain(_green_points.iter()).for_each(|p| { 
        hash.insert(*p); 
    });

    let (part1, part2) = find_largest_rectangle(&_red_points, &hash);

    println!("{DAY} part1: {}", part1);
    println!("{DAY} part2: {}", part2);
    println!("Elapsed: {}us", start.elapsed().as_micros());
}
