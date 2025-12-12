use std::collections::{HashSet, VecDeque};
use std::hash::Hash;
use std::time::Instant;
use aoc_utils::read_input;

const BASE: &str = env!("CARGO_MANIFEST_DIR");
const DAY: &str = "d09";

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: isize,
    y: isize
}

struct Rectangle {
    c1: Point,
    c2: Point,
    sides: Vec<Vec<Point>>
}

struct Matrix {
    origin: Point,
    width: usize,
    height: usize,
    cells: Vec<Vec<bool>>,
}

impl Matrix {
    fn from_points(points: &HashSet<Point>) -> Matrix {
        if points.is_empty() {
            return Matrix { origin: Point { x: 0, y: 0 }, width: 0, height: 0, cells: Vec::new() };
        }

        let mut min_x = isize::MAX;
        let mut max_x = isize::MIN;
        let mut min_y = isize::MAX;
        let mut max_y = isize::MIN;

        for point in points.iter() {
            min_x = min_x.min(point.x);
            max_x = max_x.max(point.x);
            min_y = min_y.min(point.y);
            max_y = max_y.max(point.y);
        }

        let width = (max_x - min_x + 1) as usize;
        let height = (max_y - min_y + 1) as usize;
        let mut cells = vec![vec![false; width]; height];

        for point in points.iter() {
            let x = (point.x - min_x) as usize;
            let y = (point.y - min_y) as usize;
            cells[y][x] = true;
        }

        Matrix {
            origin: Point { x: min_x, y: min_y },
            width,
            height,
            cells,
        }
    }

    fn print(&self) {
        if self.cells.is_empty() {
            println!("(empty matrix)");
            return;
        }

        for y in (0..self.height).rev() {
            for x in 0..self.width {
                let ch = if self.cells[y][x] { '#' } else { '.' };
                print!("{ch}");
            }
            println!();
        }
    }
}

impl Rectangle {
    fn new(c1: Point, c2: Point) -> Rectangle {
        Rectangle { c1, c2, sides: Vec::new() }
    }

    fn print(&self) {
        println!("({} {}), ({} {})", self.c1.x, self.c1.y, self.c2.x, self.c2.y)
    }
    
    fn area(&self) -> isize {
        let width = (self.c2.x - self.c1.x+1).abs();
        let height = (self.c2.y - self.c1.y+1).abs();
        width * height
    }

    fn get_perimeter(&mut self) {
        let mut sides: Vec<Vec<Point>> = vec![Vec::new(), Vec::new(), Vec::new(), Vec::new()];

        let xmin = self.c1.x.min(self.c2.x);
        let xmax = self.c1.x.max(self.c2.x);
        
        let ymax = self.c1.y.max(self.c2.y);
        let ymin = self.c1.y.min(self.c2.y);

        for x in xmin..(xmax+1) {
            sides[0].push(Point { x, y: ymin });
            sides[1].push(Point { x, y: ymax });
        }

        for y in ymin..(ymax+1) {
            sides[2].push(Point { x: xmin, y });
            sides[3].push(Point { x: xmax, y });
        }

        self.sides = sides;
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
    let mut cp = red_points.last().unwrap();
    for i in 0..red_points.len() {
        if cp.x == red_points[i].x {
            let y_start = cp.y.min(red_points[i].y);
            let y_end = cp.y.max(red_points[i].y);
            for y in (y_start+1)..y_end {
                green_points.push(Point { x: cp.x, y});
            }
        } else if cp.y == red_points[i].y {
            let x_start = cp.x.min(red_points[i].x);
            let x_end = cp.x.max(red_points[i].x);
            for x in (x_start+1)..x_end {
                green_points.push(Point { x, y: cp.y});
            }
        }
        cp = &red_points[i]
    }
    green_points
}

fn fill_area(hash: &mut HashSet<Point>) -> HashSet<Point> {
    let matrix = Matrix::from_points(hash);
    if matrix.cells.is_empty() {
        return hash.clone();
    }

    let mut visited = vec![vec![false; matrix.width]; matrix.height];
    let mut queue = VecDeque::new();

    let mut push_outside = |x: usize, y: usize| {
        if !visited[y][x] && !matrix.cells[y][x] {
            visited[y][x] = true;
            queue.push_back((x, y));
        }
    };

    for x in 0..matrix.width {
        push_outside(x, 0);
        if matrix.height > 1 {
            push_outside(x, matrix.height - 1);
        }
    }

    for y in 0..matrix.height {
        push_outside(0, y);
        if matrix.width > 1 {
            push_outside(matrix.width - 1, y);
        }
    }

    while let Some((x, y)) = queue.pop_front() {
        for (dx, dy) in [(1isize, 0), (-1isize, 0), (0isize, 1), (0isize, -1)] {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx < 0 || ny < 0 {
                continue;
            }
            let nx = nx as usize;
            let ny = ny as usize;
            if nx < matrix.width && ny < matrix.height && !visited[ny][nx] && !matrix.cells[ny][nx] {
                visited[ny][nx] = true;
                queue.push_back((nx, ny));
            }
        }
    }

    let mut interior_points = Vec::new();
    for y in 0..matrix.height {
        for x in 0..matrix.width {
            if !matrix.cells[y][x] && !visited[y][x] {
                interior_points.push(Point {
                    x: matrix.origin.x + x as isize,
                    y: matrix.origin.y + y as isize,
                });
            }
        }
    }

    interior_points.iter().for_each(|p| {
        hash.insert(*p);
    });

    hash.clone()
}

fn is_area_possible(rect: &mut Rectangle, hash: &HashSet<Point>) -> bool {
    rect.get_perimeter();
    let mut possible_sides = 0;
    for side in rect.sides.iter() {
        if side.iter().all(|p| hash.contains(p)) {
            possible_sides += 1;
        }
    }

    possible_sides >= 4
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
                let mut rect = Rectangle::new(
                    Point { x: p1.x.min(p2.x), y: p1.y.min(p2.y)},
                    Point { x: p1.x.max(p2.x), y: p1.y.max(p2.y)}
                );
                let area = rect.area();
                if area > largest_area_1 {
                    largest_area_1 = area;
                }
                if area > largest_area_2 && is_area_possible(&mut rect, hash) {
                    // rect.print();
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
    let mut _red_hash: HashSet<Point> = HashSet::<Point>::new();
    _red_points.iter().for_each(|p| { 
        hash.insert(*p); 
        _red_hash.insert(*p); 
    });
    _green_points.iter().for_each(|p| { 
        hash.insert(*p); 
    });

    let matrix = Matrix::from_points(&hash);

    let filled_hash = fill_area(&mut hash);

    let (part1, part2) = find_largest_rectangle(&_red_points, &filled_hash);

    println!("{DAY} part1: {}", part1);
    println!("{DAY} part2: {}", part2);
    println!("Elapsed: {}us", start.elapsed().as_micros());
}
