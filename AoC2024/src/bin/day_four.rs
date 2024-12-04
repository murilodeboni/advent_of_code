mod utils;

use utils::input::read_lines;

fn main() {
    let input = read_lines("./src/bin/inputs/day_four.txt");

    let mut input_matrix: Vec<Vec<String>> = Vec::new();

    for i in input {
        let v: Vec<String> = i.chars().map(|c| c.to_string()).collect();
        input_matrix.push(v);
    }

    let i_max = input_matrix.len();
    let j_max = input_matrix[0].len();

    let mut part_1: usize = 0;
    let mut part_2: usize = 0;

    for i in 0..i_max {
        for j in 0..j_max {
            if input_matrix[i][j] == "A" {
                part_2 += check_for_x_mas(&input_matrix, i, j, i_max, j_max);
            }
            if input_matrix[i][j] == "X" {
                part_1 += check_for_xmas(&input_matrix, i, j, i_max, j_max);
            }
        }
    }

    println!("{}", part_1);
    println!("{}", part_2);

}

fn check_for_xmas(matrix: &Vec<Vec<String>>, i: usize, j: usize, i_max: usize, j_max: usize) -> usize {
    let mut count:usize = 0;
    count += check_horizontal(matrix, i, j, i_max, j_max);
    count += check_vertical(matrix, i, j, i_max, j_max);
    count += check_diagonal(matrix, i, j, i_max, j_max);
    count
}

fn check_horizontal(matrix: &Vec<Vec<String>>, i: usize, j: usize, i_max: usize, j_max: usize) -> usize {
    let mut count:usize = 0;
    if j+3 < j_max && matrix[i][j+1] == "M" && matrix[i][j+2] == "A" && matrix[i][j+3] == "S" {
        count += 1;
        
    }
    if j >= 3 && matrix[i][j-1] == "M" && matrix[i][j-2] == "A" && matrix[i][j-3] == "S" {
        count += 1;
    } 
    count
}

fn check_vertical(matrix: &Vec<Vec<String>>, i: usize, j: usize, i_max: usize, j_max: usize) -> usize {
    let mut count:usize = 0;
    if i+3 < j_max && matrix[i+1][j] == "M" && matrix[i+2][j] == "A" && matrix[i+3][j] == "S" {
        count += 1;
    }
    if i >= 3 && matrix[i-1][j] == "M" && matrix[i-2][j] == "A" && matrix[i-3][j] == "S" {
        count += 1;
    } 
    count
}

fn check_diagonal(matrix: &Vec<Vec<String>>, i: usize, j: usize, i_max: usize, j_max: usize) -> usize {
    // println!("{} {}", i, j);

    let mut count:usize = 0;
    // down right
    if i+3 < i_max && j+3 < j_max && matrix[i+1][j+1] == "M" && matrix[i+2][j+2] == "A" && matrix[i+3][j+3] == "S" {
        count += 1;
    }
    // up left
    if i >= 3 && j >= 3 && matrix[i-1][j-1] == "M" && matrix[i-2][j-2] == "A" && matrix[i-3][j-3] == "S" {
        count += 1;
    } 
    // down left
    if i+3 < i_max && j >= 3 && matrix[i+1][j-1] == "M" && matrix[i+2][j-2] == "A" && matrix[i+3][j-3] == "S" {
        count += 1;
    } 
    // up right
    if i >= 3 && j+3 < j_max && matrix[i-1][j+1] == "M" && matrix[i-2][j+2] == "A" && matrix[i-3][j+3] == "S" {
        count += 1;
    }
    count
}

fn check_for_x_mas(matrix: &Vec<Vec<String>>, i: usize, j: usize, i_max: usize, j_max: usize) -> usize {
    let mut count:usize = 0;

    let combs = vec![vec!["M","S","M","S"],vec!["S","M","M","S"],vec!["M","S","S","M"],vec!["S","M","S","M"]];

    for comb in combs {
        // up left
        if (i >= 1 && j >= 1 && matrix[i-1][j-1] == comb[0])
        // down right
        && (i+1 < i_max && j+1 < j_max && matrix[i+1][j+1] == comb[1])
        // up right
        && (i >= 1 && j+1 < j_max && matrix[i-1][j+1] == comb[2]) 
        // down left
        && (i+1 < i_max && j >= 1 && matrix[i+1][j-1] == comb[3]) {
            count += 1;
            // println!("{} {}", i, j);
        }
    }

    count
}