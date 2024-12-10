use std::{cmp::Ordering, collections::HashMap, error::Error, fs};

pub fn five() -> Result<(), Box<dyn Error>> {
    println!("Day Five");

    let pt1 = five_one()?;
    println!("Part 1: {}", pt1);

    let pt2 = five_two()?;
    println!("Part 2: {}", pt2);

    Ok(())
}

fn five_one() -> Result<String, Box<dyn Error>> {
    let file = fs::read_to_string("inputs/five.txt")?;

    let sections: Vec<&str> = file.split("\r\n\r\n").collect();

    let mut rule_map: HashMap<i32, Vec<i32>> = HashMap::new();

    let rules = sections[0];
    let pages = sections[1];

    for ele in rules.lines() {
        let nums: Vec<i32> = ele.split("|").filter_map(|f| f.parse().ok()).collect();

        let left = nums[0];
        let right = nums[1];

        match rule_map.get_mut(&left) {
            Some(vec) => {
                vec.push(right);
            }
            None => {
                rule_map.insert(left, vec![right]);
            }
        }
    }

    let mut sum = 0;

    'outer: for line in pages.lines() {
        let nums: Vec<i32> = line.split(",").filter_map(|f| f.parse().ok()).collect();
        for (idx, num) in nums.iter().enumerate() {
            match rule_map.get(&num) {
                Some(num_rules) => {
                    let to_left = &nums[..idx];
                    for left_num in to_left {
                        if num_rules.contains(left_num) {
                            // Go To Statement Considered Harmful 🦀
                            continue 'outer;
                        }
                    }
                }
                None => {
                    continue;
                }
            }
        }
        let mid = nums.len() / 2;
        let mid_num = &nums[mid..mid + 1][0];
        sum += mid_num;
    }

    Ok(sum.to_string())
}

fn five_two() -> Result<String, Box<dyn Error>> {
    let file = fs::read_to_string("inputs/five.txt")?;

    let sections: Vec<&str> = file.split("\r\n\r\n").collect();

    let mut rule_map: HashMap<i32, Vec<i32>> = HashMap::new();

    let rules = sections[0];
    let pages = sections[1];

    for ele in rules.lines() {
        let nums: Vec<i32> = ele.split("|").filter_map(|f| f.parse().ok()).collect();

        let left = nums[0];
        let right = nums[1];

        match rule_map.get_mut(&left) {
            Some(vec) => {
                vec.push(right);
            }
            None => {
                rule_map.insert(left, vec![right]);
            }
        }
    }

    let mut sum = 0;

    for line in pages.lines() {
        let nums: Vec<i32> = line.split(",").filter_map(|f| f.parse().ok()).collect();

        let mut sorted = nums.clone();
        sorted.sort_by(|l, r| match rule_map.get(&l) {
            Some(num_rules) => {
                if num_rules.contains(r) {
                    return Ordering::Less;
                } else {
                    return Ordering::Greater;
                }
            }
            None => {
                return Ordering::Equal;
            }
        });

        if sorted != nums {
            // Array was changed
            let mid = sorted.len() / 2;
            let mid_num = &sorted[mid..mid + 1][0];
            sum += mid_num;
        }
    }

    Ok(sum.to_string())
}
