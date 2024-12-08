use std::{ collections::{ HashMap, HashSet }, error::Error, fs };

use itertools::Itertools;

pub fn eight() -> Result<(), Box<dyn Error>> {
    println!("Day Eight");

    let pt1 = eight_one()?;
    println!("Part 1: {}", pt1);

    let pt2 = eight_two()?;
    println!("Part 2: {}", pt2);

    Ok(())
}

#[derive(Debug, Clone, Copy)]
#[derive(Eq, Hash, PartialEq)]
struct Pos {
    y: i32,
    x: i32,
}

fn eight_one() -> Result<String, Box<dyn Error>> {
    let file = fs::read_to_string("inputs/eight.txt")?;
    let grid: Vec<Vec<char>> = file
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    let mut map: HashMap<char, Vec<Pos>> = HashMap::new();

    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c != '.' {
                map.entry(*c)
                    .or_insert_with(Vec::new)
                    .push(Pos { y: y as i32, x: x as i32 });
            }
        }
    }

    let mut nodes: HashSet<Pos> = HashSet::new();

    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    for (_, v) in map {
        for (l, r) in v.iter().tuple_combinations() {
            let dy = l.y - r.y;
            let dx = l.x - r.x;

            let pos_a = Pos {
                y: l.y + dy,
                x: l.x + dx,
            };
            let pos_b = Pos {
                y: r.y - dy,
                x: r.x - dx,
            };
            if pos_a.x >= 0 && pos_a.x < width && pos_a.y >= 0 && pos_a.y < height {
                nodes.insert(pos_a);
            }
            if pos_b.x >= 0 && pos_b.x < width && pos_b.y >= 0 && pos_b.y < height {
                nodes.insert(pos_b);
            }
        }
    }

    let sum = nodes.len();

    Ok(sum.to_string())
}

fn eight_two() -> Result<String, Box<dyn Error>> {
    let file = fs::read_to_string("inputs/eight.txt")?;
    let grid: Vec<Vec<char>> = file
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    let mut map: HashMap<char, Vec<Pos>> = HashMap::new();

    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c != '.' {
                map.entry(*c)
                    .or_insert_with(Vec::new)
                    .push(Pos { y: y as i32, x: x as i32 });
            }
        }
    }

    let mut nodes: HashSet<Pos> = HashSet::new();

    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    for (_, v) in map {
        for (l, r) in v.iter().tuple_combinations() {
            nodes.insert(*l);
            nodes.insert(*r);

            let dy = l.y - r.y;
            let dx = l.x - r.x;

            let gcd = gcd(dy.abs(), dx.abs());

            let mdy = dy / gcd;
            let mdx = dx / gcd;

            let mut pos_a = Pos {
                y: l.y + mdy,
                x: l.x + mdx,
            };
            while pos_a.x >= 0 && pos_a.x < width && pos_a.y >= 0 && pos_a.y < height {
                nodes.insert(pos_a);
                pos_a.y += mdy;
                pos_a.x += mdx;
            }
            let mut pos_b = Pos {
                y: r.y - mdy,
                x: r.x - mdx,
            };
            while pos_b.x >= 0 && pos_b.x < width && pos_b.y >= 0 && pos_b.y < height {
                nodes.insert(pos_b);
                pos_b.y -= mdy;
                pos_b.x -= mdx;
            }
        }
    }

    let sum = nodes.len();

    Ok(sum.to_string())
}

// from https://gist.github.com/victor-iyi/8a84185c1d52419b0d4915a648d5e3e1
pub fn gcd(mut n: i32, mut m: i32) -> i32 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}
