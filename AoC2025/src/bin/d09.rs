use std::collections::{HashMap, HashSet, VecDeque};
use std::time::Instant;
use aoc_utils::read_input;

const BASE: &str = env!("CARGO_MANIFEST_DIR");
const DAY: &str = "d09";

#[derive(PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
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



fn find_largest_rectangle(points: &Vec<Point>) -> isize {
    let mut largest_area: isize = 0;
    let mut largest_rectangle = Rectangle { c1: Point { x: 0, y: 0 }, c2: Point { x: 0, y: 0 } };
    for p1 in points {
        for p2 in points {
            if p1 != p2 {
                let rect = Rectangle {
                    c1: Point { x: p1.x.min(p2.x), y: p1.y.min(p2.y) },
                    c2: Point { x: p1.x.max(p2.x), y: p1.y.max(p2.y) },
                };
                let area = rect.area();
                if area > largest_area {
                    // println!("New largest rectangle: ({}, {}) to ({}, {}) with area {}", rect.c1.x, rect.c1.y, rect.c2.x, rect.c2.y, area);
                    largest_area = area;
                    largest_rectangle = rect;
                }
            }
        }
    }
    largest_area
}
fn main() {
    let start = Instant::now();

    let part2: usize = 0;

    let input = read_input(BASE, DAY, false);

    let _points = parse_input(input);

    println!("{DAY} part1: {}", find_largest_rectangle(&_points));
    println!("{DAY} part2: {}", part2);
    println!("Elapsed: {}us", start.elapsed().as_micros());
}
