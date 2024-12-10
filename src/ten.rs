use std::{ collections::HashSet, error::Error, fs::{ self }, path };

use itertools::enumerate;
use rayon::vec;

pub fn ten() -> Result<(), Box<dyn Error>> {
    println!("Day Ten");

    let pt1 = ten_one()?;
    println!("Part 1: {}", pt1);

    let pt2 = ten_two()?;
    println!("Part 2: {}", pt2);

    Ok(())
}

#[derive(Eq, Hash, PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
struct Pos {
    y: usize,
    x: usize,
}

fn ten_one() -> Result<String, Box<dyn Error>> {
    let file = fs::read_to_string("inputs/ten.txt")?;
    let map: Vec<Vec<u32>> = file
        .lines()
        .map(|l|
            l
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        )
        .collect();

    let mut sum = 0;

    // find trailheads
    for (y, r) in map.iter().enumerate() {
        for (x, n) in r.iter().enumerate() {
            if *n == 0 {
                let mut seen = HashSet::new();
                let curr_pos = Pos { y, x };
                explore(curr_pos, &map, &mut seen);
                sum += seen.len();
            }
        }
    }

    Ok(sum.to_string())
}

fn explore(curr_pos: Pos, map: &Vec<Vec<u32>>, seen: &mut HashSet<Pos>) {
    let curr = map[curr_pos.y][curr_pos.x];
    let mut dirs: Vec<Pos> = vec![
        Pos { y: curr_pos.y + 1, x: curr_pos.x },
        Pos { y: curr_pos.y, x: curr_pos.x + 1 }
    ];
    if let Some(y) = curr_pos.y.checked_sub(1) {
        dirs.push(Pos { y, x: curr_pos.x });
    }
    if let Some(x) = curr_pos.x.checked_sub(1) {
        dirs.push(Pos { y: curr_pos.y, x });
    }
    for dir in dirs {
        match map.get(dir.y).and_then(|r| r.get(dir.x)) {
            Some(t) => {
                if *t == curr + 1 {
                    if *t == 9 {
                        seen.insert(dir);
                    } else {
                        explore(dir, map, seen);
                    }
                }
            }
            None => {}
        }
    }
}

fn ten_two() -> Result<String, Box<dyn Error>> {
    let file = fs::read_to_string("inputs/ten.txt")?;
    let map: Vec<Vec<u32>> = file
        .lines()
        .map(|l|
            l
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        )
        .collect();

    let mut sum = 0;

    // find trailheads
    for (y, r) in map.iter().enumerate() {
        for (x, n) in r.iter().enumerate() {
            if *n == 0 {
                let curr_pos = Pos { y, x };
                let mut completed = vec![];
                explore_v2(curr_pos, &map, vec![], &mut completed);
                sum += completed.len();
            }
        }
    }

    Ok(sum.to_string())
}

fn explore_v2(curr_pos: Pos, map: &Vec<Vec<u32>>, path: Vec<Pos>, completed: &mut Vec<Vec<Pos>>) {
    let curr = map[curr_pos.y][curr_pos.x];

    let mut dirs: Vec<Pos> = vec![
        Pos { y: curr_pos.y + 1, x: curr_pos.x },
        Pos { y: curr_pos.y, x: curr_pos.x + 1 }
    ];
    if let Some(y) = curr_pos.y.checked_sub(1) {
        dirs.push(Pos { y, x: curr_pos.x });
    }
    if let Some(x) = curr_pos.x.checked_sub(1) {
        dirs.push(Pos { y: curr_pos.y, x });
    }
    for dir in dirs {
        let mut d_path = path.clone();
        match map.get(dir.y).and_then(|r| r.get(dir.x)) {
            Some(t) => {
                if *t == curr + 1 {
                    d_path.push(dir.clone());
                    if *t == 9 {
                        completed.push(d_path);
                    } else {
                        explore_v2(dir, map, d_path, completed);
                    }
                }
            }
            None => {}
        }
    }
}
