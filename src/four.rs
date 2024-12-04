use std::{ error::Error, fs };

pub fn four() -> Result<(), Box<dyn Error>> {
    println!("Day Four");

    let pt1 = four_one()?;
    println!("Part 1: {}", pt1);

    let pt2 = four_two()?;
    println!("Part 2: {}", pt2);

    Ok(())
}

fn four_one() -> Result<String, Box<dyn Error>> {
    let file = fs::read_to_string("inputs/four.txt")?;

    let matrix: Vec<Vec<char>> = file
        .split("\n")
        .map(|l| l.chars().collect())
        .collect();

    let mut sum = 0;

    for (y, row) in matrix.iter().enumerate() {
        for (x, el) in row.iter().enumerate() {
            if *el == 'X' {
                let y: i32 = y.try_into()?;
                let x: i32 = x.try_into()?;
                let search: Vec<i32> = vec![-1, 0, 1];
                for dy in &search {
                    for dx in &search {
                        if *dy == 0 && *dx == 0 {
                            continue;
                        }
                        if search_direction(&matrix, (y, x), (*dy, *dx), 'X')? {
                            sum += 1;
                        }
                    }
                }
            }
        }
    }

    Ok(sum.to_string())
}

fn search_direction(
    matrix: &Vec<Vec<char>>,
    (y, x): (i32, i32),
    (dy, dx): (i32, i32),
    prev_char: char
) -> Result<bool, Box<dyn Error>> {
    let syi = y + dy;
    let sxi = x + dx;
    if syi < 0 || sxi < 0 {
        return Ok(false);
    }
    let sy: usize = syi.try_into()?;
    let sx: usize = sxi.try_into()?;
    if let Some(next_row) = matrix.get(sy) {
        if let Some(next_char) = next_row.get(sx) {
            if prev_char == 'X' && *next_char == 'M' {
                return search_direction(matrix, (syi, sxi), (dy, dx), 'M');
            } else if prev_char == 'M' && *next_char == 'A' {
                return search_direction(matrix, (syi, sxi), (dy, dx), 'A');
            } else if prev_char == 'A' && *next_char == 'S' {
                return Ok(true);
            }
        }
    }

    Ok(false)
}

fn four_two() -> Result<String, Box<dyn Error>> {
    let file = fs::read_to_string("inputs/four.txt")?;

    let matrix: Vec<Vec<char>> = file
        .split("\n")
        .map(|l| l.chars().collect())
        .collect();

    let mut sum = 0;

    for (y, row) in matrix.iter().enumerate() {
        for (x, el) in row.iter().enumerate() {
            if *el == 'A' {
                let y: i32 = y.try_into()?;
                let x: i32 = x.try_into()?;
                let search: Vec<i32> = vec![-1, 1];
                let mut prev_m: Option<(i32, i32)> = None;
                let mut prev_s: Option<(i32, i32)> = None;
                let mut has_m = false;
                let mut has_s = false;
                for dy in &search {
                    for dx in &search {
                        let syi = y + dy;
                        let sxi = x + dx;
                        if syi < 0 || sxi < 0 {
                            continue;
                        }
                        let sy: usize = syi.try_into()?;
                        let sx: usize = sxi.try_into()?;
                        if let Some(next_row) = matrix.get(sy) {
                            if let Some(next_char) = next_row.get(sx) {
                                match next_char {
                                    'M' => {
                                        match prev_m {
                                            Some((py, px)) => {
                                                if !(syi != py && sxi != px) {
                                                    has_m = true;
                                                }
                                            }
                                            None => {
                                                prev_m = Some((syi, sxi));
                                            }
                                        }
                                    }
                                    'S' => {
                                        match prev_s {
                                            Some((py, px)) => {
                                                if !(syi != py && sxi != px) {
                                                    has_s = true;
                                                }
                                            }
                                            None => {
                                                prev_s = Some((syi, sxi));
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }

                if has_m && has_s {
                    sum += 1;
                }
            }
        }
    }

    Ok(sum.to_string())
}
