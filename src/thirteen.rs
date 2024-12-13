use std::{error::Error, fs};

use anyhow::Result;
use regex::Regex;

pub fn thirteen() -> Result<(), Box<dyn Error>> {
    println!("Day Thirteen");

    println!("Part 1: {}", thirteen_one()?);
    println!("Part 2: {}", thirteen_two()?);

    Ok(())
}

fn thirteen_one() -> Result<String, Box<dyn Error>> {
    let file = fs::read_to_string("inputs/thirteen.txt")?;
    let regex_a: Regex = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)")?;
    let regex_b: Regex = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)")?;
    let regex_prize: Regex = Regex::new(r"Prize: X=(\d+), Y=(\d+)")?;

    let mut sum = 0;

    for segment in file.split("\r\n\r\n") {
        let (_, [a_x, a_y]) = regex_a.captures(segment).unwrap().extract();
        let (_, [b_x, b_y]) = regex_b.captures(segment).unwrap().extract();
        let (_, [p_x, p_y]) = regex_prize.captures(segment).unwrap().extract();

        let (a_x, a_y) = (a_x.parse::<u32>()?, a_y.parse::<u32>()?);
        let (b_x, b_y) = (b_x.parse::<u32>()?, b_y.parse::<u32>()?);
        let (p_x, p_y) = (p_x.parse::<u32>()?, p_y.parse::<u32>()?);

        let mut lowest: Option<u32> = None;
        for a in 0..100 {
            for b in 0..100 {
                if a * a_x + b * b_x == p_x && a * a_y + b * b_y == p_y {
                    let local_sum = a * 3 + b * 1;
                    if let Some(l) = lowest {
                        if local_sum < l {
                            lowest = Some(local_sum);
                        }
                    } else {
                        lowest = Some(local_sum);
                    }
                }
            }
        }

        if let Some(l) = lowest {
            sum += l;
        }
    }

    Ok(sum.to_string())
}

fn thirteen_two() -> Result<String, Box<dyn Error>> {
    let file = fs::read_to_string("inputs/thirteen.txt")?;
    let regex_a: Regex = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)")?;
    let regex_b: Regex = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)")?;
    let regex_prize: Regex = Regex::new(r"Prize: X=(\d+), Y=(\d+)")?;

    let mut sum = 0;

    for segment in file.split("\r\n\r\n") {
        let (_, [a_x, a_y]) = regex_a.captures(segment).unwrap().extract();
        let (_, [b_x, b_y]) = regex_b.captures(segment).unwrap().extract();
        let (_, [p_x, p_y]) = regex_prize.captures(segment).unwrap().extract();

        let (a0, a1) = (a_x.parse::<i64>()?, a_y.parse::<i64>()?);
        let (b0, b1) = (b_x.parse::<i64>()?, b_y.parse::<i64>()?);
        let (c0, c1) = (
            p_x.parse::<i64>()? + 10000000000000,
            p_y.parse::<i64>()? + 10000000000000,
        );

        let x = (c0 * b1 - b0 * c1) as f64 / (a0 * b1 - b0 * a1) as f64;
        let y = (a0 * c1 - c0 * a1) as f64 / (a0 * b1 - b0 * a1) as f64;
        let solution = (x.fract() == 0.0 && y.fract() == 0.0).then_some((x * 3.0 + y) as i64);

        if let Some(sol) = solution {
            sum += sol;
        }
    }

    Ok(sum.to_string())
}
