mod utils;

use utils::input::read_lines;

use std::time::Instant;

#[derive(Debug, PartialEq)]
enum SpaceType {
    File,
    Free
}

struct Space {
    space_type: SpaceType,
    value: Option<usize>,
    size: usize
}

struct Disk {
    disk_map: Vec<Space>
}

impl Disk {
    fn fragment(&mut self) {
        let mut j = self.disk_map.len() - 1;
        while j > 0 {
            // self.debug();
            if self.disk_map[j].space_type == SpaceType::File {
                let mut i = 0;
                while i < j {
                    // there are probably smarter ways to do this but it works
                    if self.disk_map[i].space_type == SpaceType::Free && self.disk_map[i].size >= self.disk_map[j].size {
                        if self.disk_map[i].size == self.disk_map[j].size {
                            self.disk_map[i].space_type = SpaceType::File;
                            self.disk_map[i].value = self.disk_map[j].value;
                            self.disk_map[j].space_type = SpaceType::Free;
                            self.disk_map[j].value = None;
                        } else {
                            self.disk_map.insert(i + 1, Space {
                                space_type: SpaceType::Free,
                                value: None,
                                size: self.disk_map[i].size - self.disk_map[j].size
                            });
                            self.disk_map[i].size = self.disk_map[j+1].size;
                            self.disk_map[i].space_type = SpaceType::File;
                            self.disk_map[i].value = self.disk_map[j+1].value;
                            self.disk_map[j+1].space_type = SpaceType::Free;
                            self.disk_map[j+1].value = None;
                        }
                        i = j;
                    } else {
                        i += 1;
                    }
                }
            }
            j -= 1;
        }
    }


    fn debug(&self) {
        for i in 0..self.disk_map.len() {
            if let Some(v) = self.disk_map[i].value {
                print!("{}", vec![v.to_string(); self.disk_map[i].size].join(""));
            } else {
                print!("{}", vec!["."; self.disk_map[i].size].join(""));

            }    
        }
        println!();
    }

    fn ans(&self) -> Vec<usize> {
        let mut ans = Vec::new();
        let mut n: i32 = 0;
        for i in 0..self.disk_map.len() {
            if let Some(v) = self.disk_map[i].value {
                ans.push(vec![v; self.disk_map[i].size]);
            } else {
                ans.push(vec![0; self.disk_map[i].size]);
            }
        }
        ans.concat()
    }
}

fn main() {
    let start = Instant::now();
    let input: Vec<String> = read_lines("./src/bin/inputs/day_09.txt");
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

    println!("part 1 - {:?} took {}ms", calculate_ans(part_1(&file_size, &free_space)), {start.elapsed().as_millis()});
    println!("part 2 - {:?} took {}ms", calculate_ans(part_2(&file_size, &free_space)), {start.elapsed().as_millis()});
}

fn part_2(file_size: &Vec<(usize, usize)>, free_space: &Vec<usize>) -> Vec<usize> {
    let mut disk: Disk = Disk {disk_map: Vec::new()};

    for i in 0..free_space.len() {
        let file = Space{
            space_type: SpaceType::File,
            value: Some(file_size[i].0),
            size: file_size[i].1
        };
        let free_space = Space{
            space_type: SpaceType::Free,
            value: None,
            size: free_space[i]
        };
        disk.disk_map.push(file);
        disk.disk_map.push(free_space);
    }
    disk.disk_map.push(Space{
        space_type: SpaceType::File,
        value: Some(file_size[file_size.len()-1].0),
        size: file_size[file_size.len()-1].1
    });

    disk.fragment();
    disk.ans()
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

fn calculate_ans(v: Vec<usize>) -> usize {
    let mut ans: usize = 0;
    for i in 0..v.len() {
        ans += i*v[i];
    }
    ans
}