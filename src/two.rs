use std::{error::Error, fs};

pub fn two() -> Result<(), Box<dyn Error>> {
    println!("Day Two");

    let pt1 = two_one()?;
    println!("Part 1: {}", pt1);

    let pt2 = two_two()?;
    println!("Part 2: {}", pt2);

    Ok(())
}

fn two_one() -> Result<String, Box<dyn Error>> {
    // Read the input to a string
    let file = fs::read_to_string("inputs/two.txt")?;

    let mut safe_count: i32 = 0;

    for ele in file.split("\n") {
        let nums: Vec<i32> = ele.split(" ").filter_map(|s| s.parse().ok()).collect();

        let is_safe = is_safe(&nums);

        if is_safe {
            safe_count += 1
        }
    }

    Ok(safe_count.to_string())
}

fn two_two() -> Result<String, Box<dyn Error>> {
    // Read the input to a string
    let file = fs::read_to_string("inputs/two.txt")?;

    let mut safe_count: i32 = 0;

    for ele in file.split("\n") {
        let nums: Vec<i32> = ele.split(" ").filter_map(|s| s.parse().ok()).collect();

        let safe = is_safe(&nums);

        if safe {
            safe_count += 1
        } else if is_subset_safe(&nums) {
            safe_count += 1
        }
    }

    Ok(safe_count.to_string())
}

fn is_safe(nums: &Vec<i32>) -> bool {
    let a: i32 = nums[0];
    let b: i32 = nums[1];

    let increasing = b > a;

    for i in 1..nums.len() {
        let curr: i32 = nums[i];
        let prev: i32 = nums[i - 1];

        if increasing && curr < prev {
            return false;
        } else if !increasing && curr > prev {
            return false;
        } else if (prev - curr).abs() < 1 {
            return false;
        } else if (prev - curr).abs() > 3 {
            return false;
        }
    }

    true
}

fn is_subset_safe(nums: &Vec<i32>) -> bool {
    for i in 0..nums.len() {
        let mut new_nums = nums.clone();
        new_nums.remove(i);

        if is_safe(&new_nums) {
            return true;
        }
    }

    false
}
