use std::{collections::HashSet, error::Error, fs};

use anyhow::Result;

pub fn twelve() -> Result<(), Box<dyn Error>> {
    println!("Day Twelve");

    println!("Part 1: {}", twelve_one()?);
    println!("Part 2: {}", twelve_two()?);

    Ok(())
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

impl Pos {
    fn north(&self) -> Pos {
        Pos(self.0, self.1 - 1)
    }
    fn south(&self) -> Pos {
        Pos(self.0, self.1 + 1)
    }
    fn east(&self) -> Pos {
        Pos(self.0 + 1, self.1)
    }
    fn west(&self) -> Pos {
        Pos(self.0 - 1, self.1)
    }
}

fn explore(curr_pos: Pos, map: &Vec<Vec<char>>, visited: &mut HashSet<Pos>) -> (i32, i32) {
    let curr = map[curr_pos.1 as usize][curr_pos.0 as usize];
    let mut perimeter = 0;
    let mut area = 1;
    let mut dirs = Vec::new();

    for (dy, dx) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let new_y = curr_pos.1 + dy;
        let new_x = curr_pos.0 + dx;

        if new_y >= 0 && new_y < map.len() as i32 && new_x >= 0 && new_x < map[0].len() as i32 {
            let new_pos = Pos(new_x, new_y);

            if map[new_pos.1 as usize][new_pos.0 as usize] != curr {
                perimeter += 1;
            } else if !visited.contains(&new_pos) {
                dirs.push(new_pos);
            }
        } else {
            perimeter += 1;
        }
    }

    visited.insert(curr_pos);

    for dir in dirs {
        if !visited.contains(&dir) {
            let (p, a) = explore(dir, map, visited);
            perimeter += p;
            area += a;
        }
    }

    (perimeter, area)
}

fn twelve_one() -> Result<String, Box<dyn Error>> {
    let file = fs::read_to_string("inputs/twelve.txt")?;
    let map: Vec<Vec<char>> = file.lines().map(|l| l.chars().collect()).collect();

    let mut total_price = 0;
    let mut visited: HashSet<Pos> = HashSet::new();

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            let curr_pos = Pos(x as i32, y as i32);
            if !visited.contains(&curr_pos) {
                let (perimeter, area) = explore(curr_pos, &map, &mut visited);
                total_price += perimeter * area;
            }
        }
    }

    Ok(total_price.to_string())
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Dir {
    North,
    East,
    West,
    South,
}

impl Dir {
    fn perpendicular(&self) -> Vec<Dir> {
        match self {
            Dir::North => vec![Dir::East, Dir::West],
            Dir::East => vec![Dir::North, Dir::South],
            Dir::South => vec![Dir::East, Dir::West],
            Dir::West => vec![Dir::North, Dir::South],
        }
    }
}

struct Grid(Vec<Vec<char>>);
impl Grid {
    fn yln(&self) -> usize {
        self.0.len()
    }
    fn xln(&self) -> usize {
        if self.yln() > 0 {
            return self.0[0].len();
        }
        0
    }
    fn is_valid(&self, p: Pos) -> bool {
        p.0 >= 0 && p.1 >= 0 && p.0 < self.xln() as i32 && p.1 < self.yln() as i32
    }
    fn at(&self, p: Pos) -> char {
        self.0[p.1 as usize][p.0 as usize]
    }
    fn get(&self, p: Pos, d: Dir) -> Option<Pos> {
        match d {
            Dir::North => Some(p.north()),
            Dir::East => Some(p.east()),
            Dir::South => Some(p.south()),
            Dir::West => Some(p.west()),
        }
        .filter(|n| self.is_valid(*n))
    }
    fn neighbors(&self, p: Pos) -> Vec<Pos> {
        let mut result = Vec::new();
        for d in [Dir::North, Dir::East, Dir::South, Dir::West] {
            if let Some(n) = self.get(p, d) {
                if self.at(n) == self.at(p) {
                    result.push(n);
                }
            }
        }
        result
    }

    fn outer_count(&self, p: Pos) -> u32 {
        let mut count = 0;
        for d in [Dir::North, Dir::East, Dir::South, Dir::West] {
            if let Some(n) = self.get(p, d) {
                if self.at(n) != self.at(p) {
                    count += 1;
                }
            } else {
                count += 1;
            }
        }
        count
    }
}

pub fn twelve_two() -> Result<String> {
    let file = fs::read_to_string("inputs/twelve.txt")?;
    let grid: Vec<Vec<char>> = file.lines().map(|l| l.chars().collect()).collect();
    let g = Grid(grid);
    let mut sum = 0;
    let mut visited = HashSet::new();
    for y in 0..g.yln() {
        for x in 0..g.xln() {
            let p = Pos(x as i32, y as i32);
            if visited.contains(&p) {
                continue;
            }
            let mut queue = Vec::new();
            queue.push(p);
            let mut scc = HashSet::new();
            while let Some(p) = queue.pop() {
                if visited.contains(&p) {
                    continue;
                }
                visited.insert(p);
                scc.insert(p);
                for n in g.neighbors(p) {
                    queue.push(n);
                }
            }

            let mut sides = 0;
            let mut visited_inner = HashSet::new();
            for &p in scc.iter() {
                if g.outer_count(p) == 0 {
                    continue;
                }
                for dir in [Dir::North, Dir::East, Dir::South, Dir::West] {
                    if let Some(n) = g.get(p, dir) {
                        if scc.contains(&n) {
                            continue;
                        }
                    }
                    if visited_inner.contains(&(p, dir)) {
                        continue;
                    }
                    sides += 1;
                    let mut queue = Vec::new();
                    queue.push((p, dir));
                    while let Some((p, d)) = queue.pop() {
                        if visited_inner.contains(&(p, d)) {
                            continue;
                        }
                        visited_inner.insert((p, d));
                        for pd in d.perpendicular() {
                            if let Some(n) = g.get(p, pd) {
                                if visited_inner.contains(&(n, d)) || !scc.contains(&n) {
                                    continue;
                                }
                                if let Some(nd) = g.get(n, d) {
                                    if scc.contains(&nd) {
                                        continue;
                                    }
                                }
                                queue.push((n, d))
                            }
                        }
                    }
                }
            }
            sum += sides * scc.len() as u32;
        }
    }
    Ok(sum.to_string())
}
