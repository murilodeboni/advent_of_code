mod utils;

use utils::input::read_lines;

use std::time::Instant;

fn main() {
    let start = Instant::now();
    let input: Vec<String> = read_lines("./src/bin/inputs/day_09_test.txt");
    let disk_map: Vec<usize> = input[0]
        .chars()
        .filter_map(|c| c.to_digit(10).map(|d| d as usize))
        .collect();

    
    let mut file_size: Vec<(usize, usize)> = Vec::new();
    let mut free_space: Vec<usize> = Vec::new();
    
    for i in 0..disk_map.len() {
        if i%2==0 {
            file_size.push((i/2,disk_map[i]));
        } else {
            free_space.push(disk_map[i]);
        }
    }

    // println!("{:?} {}", calculate_ans(part_1(&file_size, &free_space)), {start.elapsed().as_millis()});
    println!("{:?} {}", calculate_ans_chars(part_2(&file_size, &free_space)), {start.elapsed().as_millis()});

}


fn part_1(file_size: &Vec<(usize, usize)>, free_space: &Vec<usize>) -> Vec<usize> {
    let mut v: Vec<Option<usize>> = Vec::new();

    for i in 0..free_space.len() {
        v.append(&mut vec![Some(file_size[i].0); file_size[i].1]);
        v.append(&mut vec![None; free_space[i]]);
    }

    v.append(&mut vec![Some(file_size[file_size.len()-1].0); file_size[file_size.len()-1].1]);

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

fn part_2(file_size: &Vec<(usize, usize)>, free_space: &Vec<usize>) -> Vec<char> {
    fn create_str(value: String, times: usize) -> String {
        std::iter::repeat(value)
        .take(times)
        .collect::<String>()
    }
    
    let mut v: Vec<String> = Vec::new();

    for i in 0..free_space.len() {
        v.append(&mut vec![create_str(file_size[i].0.to_string(), file_size[i].1)]);
        v.append(&mut vec![create_str(String::from("."), file_size[i].1)]);
    }
    v.append(&mut vec![create_str(file_size[file_size.len()-1].0.to_string(), file_size[file_size.len()-1].1)]);

    // println!("vector - {:?}", v);

    let mut sv: Vec<String> = Vec::new();
    let mut rest_sv: String = String::new();
    let mut final_string = String::new();

    let mut i = 0;
    let mut j = v.len()-1;

    while i <= j {
        let i_can_be_parsed: bool = v[i].parse::<usize>().is_ok();
        if i_can_be_parsed {
            sv.push(v[i].clone());
            i += 1;
        } else {
            let j_can_be_parsed: bool = v[j].parse::<i32>().is_ok();
            if j_can_be_parsed && v[j].len() == v[i].len() {
                sv.push(v[j].clone());
                j -= 1;
                i += 1;
            } else if j_can_be_parsed && v[j].len() < v[i].len() {
                sv.push(v[j].clone());
                v[i] = create_str(String::from("."), v[i].len() - v[j].len());
                j -= 1;
            } else {
                rest_sv = v[j].clone() + &rest_sv;
                j -= 1
            }
        }

        final_string = sv.concat() + &rest_sv;
        println!("{:?}", final_string);
    }

    final_string.chars().collect()
}

fn calculate_ans_chars(v: Vec<char>) -> usize {
    let mut ans: usize = 0;
    for i in 0..v.len() {
        if v[i].is_numeric() {
            ans += i*v[i].to_digit(10).unwrap() as usize;
        }
    }
    ans
}

fn calculate_ans(v: Vec<usize>) -> usize {
    let mut ans: usize = 0;
    for i in 0..v.len() {
        ans += i*v[i];
    }
    ans
}