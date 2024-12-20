use std::{collections::HashMap, error::Error, fs};

pub fn one() -> Result<(), Box<dyn Error>> {
    println!("Day One");

    let pt1 = one_one()?;
    println!("Part 1: {}", pt1);

    let pt2 = one_two()?;
    println!("Part 2: {}", pt2);

    Ok(())
}

fn one_one() -> Result<String, Box<dyn Error>> {
    // Read the input to a string
    let file = fs::read_to_string("inputs/one.txt")?;

    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    for ele in file.split("\n") {
        let nums: Vec<&str> = ele.split("   ").collect();
        let left_num = nums[0].parse::<i32>()?;
        let right_num = nums[1].parse::<i32>()?;
        left.push(left_num);
        right.push(right_num);
    }

    left.sort();
    right.sort();

    let mut sum = 0;

    for i in 0..left.len() {
        sum += (left[i] - right[i]).abs();
    }

    Ok(sum.to_string())
}

fn one_two() -> Result<String, Box<dyn Error>> {
    // Read the input to a string
    let file = fs::read_to_string("inputs/one.txt")?;

    let mut right_map: HashMap<i32, i32> = HashMap::new();

    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    for ele in file.split("\n") {
        let nums: Vec<&str> = ele.split("   ").collect();
        let left_num = nums[0].parse::<i32>()?;
        let right_num = nums[1].parse::<i32>()?;
        left.push(left_num);
        right.push(right_num);
    }

    for ele in right {
        let total = match right_map.get(&ele) {
            Some(val) => val + 1,
            None => &0 + 1,
        };
        right_map.insert(ele, total);
    }

    let mut sum = 0;

    for ele in left {
        let popularity = match right_map.get(&ele) {
            Some(pop) => pop,
            None => &0,
        };

        sum += ele * popularity;
    }

    Ok(sum.to_string())
}
