use std::{ error::Error, fs };

use regex::Regex;

pub fn three() -> Result<(), Box<dyn Error>> {
    println!("Day Three");

    let pt1 = three_one()?;
    println!("Part 1: {}", pt1);

    let pt2 = three_two()?;
    println!("Part 2: {}", pt2);

    Ok(())
}

fn three_one() -> Result<String, Box<dyn Error>> {
    let file = fs::read_to_string("inputs/three.txt")?;
    let regex: Regex = Regex::new(r"mul\(\d+,\d+\)")?;

    let results: Vec<&str> = regex
        .find_iter(&file)
        .map(|digits| digits.as_str())
        .collect();

    let capture: Regex = Regex::new(r"mul\((?<left>\d+),(?<right>\d+)\)")?;

    let mut sum = 0;

    for ele in results {
        if let Some(captures) = capture.captures(ele) {
            if let (Some(left), Some(right)) = (captures.name("left"), captures.name("right")) {
                let left_num: i32 = left.as_str().parse()?;
                let right_num: i32 = right.as_str().parse()?;
                sum += left_num * right_num;
            }
        }
    }

    Ok(sum.to_string())
}

fn three_two() -> Result<String, Box<dyn Error>> {
    let file = fs::read_to_string("inputs/three.txt")?;
    let regex: Regex = Regex::new(r"mul\(\d+,\d+\)|don't\(\)|do\(\)")?;

    let results: Vec<&str> = regex
        .find_iter(&file)
        .map(|digits| digits.as_str())
        .collect();

    let capture: Regex = Regex::new(r"mul\((?<left>\d+),(?<right>\d+)\)")?;

    let mut sum = 0;

    let mut enabled = true;

    for ele in results {
        if ele == "don't()" {
            enabled = false;
        } else if ele == "do()" {
            enabled = true;
        } else if enabled {
            if let Some(captures) = capture.captures(ele) {
                if let (Some(left), Some(right)) = (captures.name("left"), captures.name("right")) {
                    let left_num: i32 = left.as_str().parse()?;
                    let right_num: i32 = right.as_str().parse()?;
                    sum += left_num * right_num;
                }
            }
        }
    }

    Ok(sum.to_string())
}
