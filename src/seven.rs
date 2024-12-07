use std::{ collections::HashMap, error::Error, fs, ops::{ Add, Mul } };

use rayon::iter::{ IntoParallelIterator, IntoParallelRefIterator, ParallelIterator };

pub fn seven() -> Result<(), Box<dyn Error>> {
    println!("Day Seven");

    let pt1 = seven_one()?;
    println!("Part 1: {}", pt1);

    let pt2 = seven_two()?;
    println!("Part 2: {}", pt2);

    Ok(())
}

fn seven_one() -> Result<String, Box<dyn Error>> {
    let file = fs::read_to_string("inputs/seven.txt")?;
    let sides: Vec<(u64, Vec<u64>)> = file
        .lines()
        .filter_map(|l| {
            let ex: Vec<&str> = l.split(": ").collect();
            let left = ex[0].parse::<u64>().ok()?;
            let right: Vec<u64> = ex[1]
                .split(" ")
                .filter_map(|n| n.parse::<u64>().ok())
                .collect();
            Some((left, right))
        })
        .collect();

    let sum: u64 = sides
        .into_par_iter()
        .map(|(p_sum, parts)| {
            let size = parts.len() - 1;
            for i in 0..(2_u64).pow(size as u32) {
                let mut local_sum: u64 = parts[0];
                for (j, r) in parts[1..].iter().enumerate() {
                    // check if j-th bit is 0
                    if ((i >> j) & 1) == 0 {
                        local_sum += r;
                    } else {
                        local_sum *= r;
                    }
                    if local_sum > p_sum {
                        break;
                    }
                }
                if p_sum == local_sum {
                    return local_sum;
                }
            }
            0
        })
        .sum();

    Ok(sum.to_string())
}

fn seven_two() -> Result<String, Box<dyn Error>> {
    let base = 3_u64;
    let file = fs::read_to_string("inputs/seven.txt")?;

    let sides: Vec<(u64, Vec<u64>)> = file
        .lines()
        .filter_map(|l| {
            let ex: Vec<&str> = l.split(": ").collect();
            let left = ex[0].parse::<u64>().ok()?;
            let right: Vec<u64> = ex[1]
                .split(" ")
                .filter_map(|n| n.parse::<u64>().ok())
                .collect();
            Some((left, right))
        })
        .collect();

    let max_size = sides
        .par_iter()
        .map(|(_, parts)| parts.len())
        .max()
        .unwrap();

    let map: HashMap<usize, Vec<Vec<u64>>> = (0..max_size)
        .into_par_iter()
        .map(|size| {
            let mut combinations = Vec::new();
            for i in 0..base.pow(size as u32) {
                let mut combination = Vec::with_capacity(size);
                let mut num = i;
                for _ in 0..size {
                    combination.push(num % base);
                    num /= base;
                }
                combinations.push(combination);
            }
            (size, combinations)
        })
        .collect();

    let sum: u64 = sides
        .into_iter()
        .map(|(p_sum, parts)| {
            let size = parts.len() - 1;
            let combinations = map.get(&size).unwrap();

            let sum = combinations
                .par_iter() // Parallelize the loop over combinations
                .find_map_any(|combination| {
                    let mut local_sum: u64 = parts[0];
                    for i in 1..parts.len() {
                        let r = parts[i];
                        let op = combination[i - 1];
                        match op {
                            0 => {
                                local_sum = local_sum.add(r);
                            }
                            1 => {
                                local_sum = local_sum.mul(r);
                            }
                            2 => {
                                local_sum = (local_sum.to_string() + &r.to_string())
                                    .parse::<u64>()
                                    .unwrap();
                            }
                            _ => panic!("Unexpected operation {}", op),
                        }
                        if local_sum > p_sum {
                            return None;
                        }
                    }

                    if p_sum == local_sum {
                        return Some(local_sum);
                    } else {
                        None
                    }
                })
                .unwrap_or(0);
            sum
        })
        .sum();

    Ok(sum.to_string())
}
