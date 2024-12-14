use std::{error::Error, fs};

use anyhow::Result;
use itertools::Itertools;
use regex::Regex;

pub fn fourteen() -> Result<(), Box<dyn Error>> {
    println!("Day Fourteen");

    println!("Part 1: {}", fourteen_one()?);
    println!("Part 2: {}", fourteen_two()?);

    Ok(())
}

#[derive(Debug)]
struct Bot {
    y: u32,
    x: u32,
    v_y: i32,
    v_x: i32,
}

fn fourteen_one() -> Result<String, Box<dyn Error>> {
    let file = fs::read_to_string("inputs/fourteen.txt")?;
    let regex: Regex = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)")?;

    let mut bots: Vec<Bot> = vec![];

    let width: u32 = 101;
    let height: u32 = 103;

    for segment in file.lines() {
        let (_, v) = regex.captures(segment).unwrap().extract();
        let [p_x, p_y, v_x, v_y] = v.map(|s| s.parse::<i32>().unwrap());

        bots.push(Bot {
            y: p_y as u32,
            x: p_x as u32,
            v_y,
            v_x,
        });
    }

    let steps = 100;

    bots.iter_mut().for_each(|b| {
        b.x = ((b.x as i32 + b.v_x * steps).rem_euclid(width as i32)) as u32;
        b.y = ((b.y as i32 + b.v_y * steps).rem_euclid(height as i32)) as u32;
    });

    let mut quad_a = 0;
    let mut quad_b = 0;
    let mut quad_c = 0;
    let mut quad_d = 0;

    bots.iter().for_each(|b| {
        let mid_x = width / 2 as u32;
        let mid_y = height / 2 as u32;
        if b.x < mid_x && b.y < mid_y {
            quad_a += 1;
        } else if b.x > mid_x && b.y < mid_y {
            quad_b += 1;
        } else if b.x > mid_x && b.y > mid_y {
            quad_c += 1;
        } else if b.x < mid_x && b.y > mid_y {
            quad_d += 1;
        }
    });

    let sum = quad_a * quad_b * quad_c * quad_d;

    Ok(sum.to_string())
}

fn fourteen_two() -> Result<String, Box<dyn Error>> {
    let file = fs::read_to_string("inputs/fourteen.txt")?;
    let regex: Regex = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)")?;

    let mut bots: Vec<Bot> = vec![];

    let width: u32 = 101;
    let height: u32 = 103;

    for segment in file.lines() {
        let (_, v) = regex.captures(segment).unwrap().extract();
        let [p_x, p_y, v_x, v_y] = v.map(|s| s.parse::<i32>().unwrap());

        bots.push(Bot {
            y: p_y as u32,
            x: p_x as u32,
            v_y,
            v_x,
        });
    }

    for i in 0..10_000 {
        bots.iter_mut().for_each(|b| {
            b.x = ((b.x as i32 + b.v_x).rem_euclid(width as i32)) as u32;
            b.y = ((b.y as i32 + b.v_y).rem_euclid(height as i32)) as u32;
        });

        let unique = bots.iter().map(|b| (b.y, b.x)).unique().count();

        if unique > 499 {
            return Ok((i + 1).to_string());
        }
    }

    Ok("None Found".to_string())
}
