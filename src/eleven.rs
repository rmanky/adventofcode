use std::{
    collections::HashMap,
    error::Error,
    fs::{self},
};

use anyhow::Result;
use itertools::Itertools;

pub fn eleven() -> Result<(), Box<dyn Error>> {
    println!("Day Eleven");

    let pt1 = eleven_one()?;
    println!("Part 1: {}", pt1);

    let pt1 = eleven_two()?;
    println!("Part 1: {}", pt1);

    Ok(())
}

fn num_digits(mut num: u64) -> u32 {
    let mut count = 0;
    while num > 0 {
        num /= 10;
        count += 1;
    }
    count
}

fn eleven_one() -> Result<String, Box<dyn Error>> {
    let file = fs::read_to_string("inputs/eleven.txt")?;
    let mut nums: Vec<u64> = file
        .split(" ")
        .filter_map(|c| c.parse::<u64>().ok())
        .collect();

    let powers_of_ten: Vec<u64> = (0..=10).map(|i| 10_u64.pow(i)).collect();

    for _ in 0..25 {
        let r = nums
            .iter()
            .flat_map(|num| {
                if *num == 0 {
                    vec![1]
                } else {
                    let size: u32 = num_digits(*num);
                    if size % 2 == 0 {
                        let mid = size / 2;
                        let divisor = powers_of_ten[mid as usize];
                        let left = num / divisor;
                        let right = num - (left * divisor);
                        vec![left, right]
                    } else {
                        vec![num * 2024]
                    }
                }
            })
            .collect();
        {}
        nums = r;
    }

    let sum = nums.len();

    Ok(sum.to_string())
}

fn blink(n: u64) -> Vec<u64> {
    let num_n = num_digits(n);
    match n {
        0 => vec![1],
        n if num_n % 2 == 0 => {
            let size = num_n;
            let mid = size / 2;
            let divisor = 10_u64.pow(mid);
            let left = n / divisor;
            let right = n - (left * divisor);
            vec![left, right]
        }
        n => vec![n * 2024],
    }
}

fn eleven_two() -> Result<String> {
    let mut counter = fs::read_to_string("inputs/eleven.txt")?
        .trim()
        .split(' ')
        .map(|f| f.parse().unwrap())
        .counts();

    for _ in 0..75 {
        counter = counter
            .iter()
            .flat_map(|(&n, &c)| blink(n).into_iter().map(move |b| (b, c)))
            .fold(HashMap::new(), |mut acc, (b, c)| {
                *acc.entry(b).or_insert(0) += c;
                acc
            });
    }

    let sum = counter.values().sum::<usize>();

    Ok(sum.to_string())
}
