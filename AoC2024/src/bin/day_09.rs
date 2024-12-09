mod utils;

use utils::input::read_lines;

use std::time::Instant;

fn main() {
    let start = Instant::now();
    let input: Vec<String> = read_lines("./src/bin/inputs/day_09.txt");
    let disk_map: Vec<usize> = input[0]
        .chars()
        .filter_map(|c| c.to_digit(10).map(|d| d as usize))
        .collect();

    println!("{:?} {}", part1(disk_map), {start.elapsed().as_millis()});


}

fn part1(disk_map: Vec<usize>) -> usize {
    let mut file_size: Vec<(usize, usize)> = Vec::new();
    let mut free_space: Vec<usize> = Vec::new();

    for i in 0..disk_map.len() {
        if i%2==0 {
            file_size.push((i/2,disk_map[i]));
        } else {
            free_space.push(disk_map[i]);
        }
    }

    let mut final_vec: Vec<Option<usize>> = Vec::new();

    for i in 0..free_space.len() {
        final_vec.append(&mut vec![Some(file_size[i].0); file_size[i].1]);
        final_vec.append(&mut vec![None; free_space[i]]);
    }
    final_vec.append(&mut vec![Some(file_size[file_size.len()-1].0); file_size[file_size.len()-1].1]);

    calculate_part_1_ans(custom_sort(final_vec))
}

fn custom_sort(v: Vec<Option<usize>>) -> Vec<usize> {
    // println!("vector - {:?}", v);
    let mut sv: Vec<usize> = Vec::new();
    let mut i = 0;
    let mut j = v.len()-1;
    while i <= j {
        if let Some(ni) = v[i] {
            sv.push(ni);
            i += 1;
        } else {
            if let Some(nf) = v[j] {
                sv.push(nf);
                j -= 1;
                i += 1;
            } else {
                j -= 1
            }
        }
    }
    sv
}

fn calculate_part_1_ans(v: Vec<usize>) -> usize {
    let mut ans: usize = 0;
    for i in 0..v.len() {
        ans += i*v[i];
    }
    ans
}