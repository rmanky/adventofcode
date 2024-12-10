use std::{ collections::{ HashMap, HashSet }, error::Error, fs::{ self, remove_dir } };

use itertools::Itertools;

pub fn nine() -> Result<(), Box<dyn Error>> {
    println!("Day Nine");

    let pt1 = nine_one()?;
    println!("Part 1: {}", pt1);

    let pt2 = nine_two()?;
    println!("Part 2: {}", pt2);

    Ok(())
}

#[derive(Debug, Clone, Copy)]
#[derive(Eq, Hash, PartialEq)]
struct Pos {
    y: i32,
    x: i32,
}

fn nine_one() -> Result<String, Box<dyn Error>> {
    let file = fs::read_to_string("inputs/nine.txt")?;
    let mut chars: Vec<i32> = file
        .chars()
        .filter(|c| c.is_digit(10))
        .filter_map(|c| c.to_digit(10))
        .enumerate()
        .map(|(i, c)| {
            if i % 2 == 0 {
                let num = (i as i32) / 2;
                let t: Vec<i32> = (0..c)
                    .into_iter()
                    .map(|_| num as i32)
                    .collect();
                t
            } else {
                let t: Vec<i32> = (0..c)
                    .into_iter()
                    .map(|_| -1)
                    .collect();
                t
            }
        })
        .flatten()
        .collect();

    let spaces = chars
        .iter()
        .filter(|c| **c == -1)
        .count();

    for _ in 0..spaces {
        let last = chars.pop().unwrap();
        let insert = chars
            .iter()
            .position(|c| *c == -1)
            .unwrap();
        chars.remove(insert);
        chars.insert(insert, last);
    }

    let sum: u64 = chars
        .iter()
        .enumerate()
        .map(|(i, c)| { (i as u64) * (*c as u64) })
        .sum();

    Ok(sum.to_string())
}

#[derive(Debug, Clone)]
struct Block {
    id: String,
    length: usize,
}

fn nine_two() -> Result<String, Box<dyn Error>> {
    let input_str = fs::read_to_string("inputs/nine.txt")?;
    let input: Vec<usize> = input_str
        .chars()
        .filter(|c| c.is_digit(10))
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    let mut disk: Vec<Block> = Vec::with_capacity(input.len() / 2); // Pre-allocate capacity
    let mut id = 0;
    for (i, &length) in input.iter().enumerate() {
        disk.push(Block {
            id: if i % 2 == 0 {
                id.to_string()
            } else {
                ".".to_string()
            },
            length,
        });
        if i % 2 == 0 {
            id += 1;
        }
    }

    for i in (0..id).rev() {
        let file = disk
            .iter()
            .position(|block| block.id == i.to_string())
            .unwrap();
        let free = disk
            .iter()
            .position(|block| block.id == "." && block.length >= disk[file].length);

        if free.is_none() || file < free.unwrap() {
            continue;
        }

        let free = free.unwrap();
        let file_length = disk[file].length;

        if disk[free].length > file_length {
            disk[free].length -= file_length;
            disk.insert(free, Block { id: i.to_string(), length: file_length });
            disk[file + 1].id = ".".to_string();
        } else {
            disk[free].id = i.to_string();
            disk[file].id = ".".to_string();
        }
    }

    let mut j = 0;
    while j < disk.len() - 1 {
        if disk[j].id == "." && disk[j + 1].id == "." {
            disk[j].length += disk.remove(j + 1).length;
            j -= 1;
        }
        j += 1;
    }

    let mut block = 0;
    let mut checksum = 0;
    for i in 0..disk.len() {
        if disk[i].id == "." {
            block += disk[i].length;
        } else {
            let id = disk[i].id.parse::<usize>().unwrap();
            checksum += (block..block + disk[i].length).sum::<usize>() * id;
            block += disk[i].length;
        }
    }

    Ok(checksum.to_string())
}
