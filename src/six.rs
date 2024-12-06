use std::{ collections::HashSet, error::Error, fs };

pub fn six() -> Result<(), Box<dyn Error>> {
    println!("Day Six");

    let pt1 = six_one()?;
    println!("Part 1: {}", pt1);

    let pt2 = six_two()?;
    println!("Part 2: {}", pt2);

    Ok(())
}

#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(Hash, Eq, PartialEq)]
struct Pos {
    y: i32,
    x: i32,
}

#[derive(Debug)]
#[derive(PartialEq)]
enum Square {
    Free,
    Box,
}
// 0 = up, 1 = right, 2 = down, 3 = left
const DIRS: [Pos; 4] = [
    Pos { y: -1, x: 0 },
    Pos { y: 0, x: 1 },
    Pos { y: 1, x: 0 },
    Pos { y: 0, x: -1 },
];

fn six_one() -> Result<String, Box<dyn Error>> {
    let file = fs::read_to_string("inputs/six.txt")?;

    let sum = run_maze(&file)?.len();

    Ok(sum.to_string())
}

fn six_two() -> Result<String, Box<dyn Error>> {
    let file = fs::read_to_string("inputs/six.txt")?;

    let candidates = run_maze(&file)?;

    let mut guard_pos = Pos {
        y: 0,
        x: 0,
    };

    let chars = vec!['.', '#', '^'];

    let mut map: Vec<Vec<Square>> = file
        .lines()
        .enumerate()
        .map(|(y, l)|
            l
                .chars()
                .filter(|c| chars.contains(c))
                .enumerate()
                .filter_map(|(x, e)| {
                    match e {
                        '#' => Some(Square::Box),
                        '^' => {
                            guard_pos.y = y.try_into().ok()?;
                            guard_pos.x = x.try_into().ok()?;
                            Some(Square::Free)
                        }
                        '.' => Some(Square::Free),
                        _ => None,
                    }
                })
                .collect()
        )
        .collect();

    let mut sum = 0;

    for ele in candidates {
        let y: usize = ele.y.try_into()?;
        let x: usize = ele.x.try_into()?;
        if map[y][x] == Square::Free {
            map[y][x] = Square::Box;
            let (loops, _) = is_loop(&map, guard_pos)?;
            if loops {
                sum += 1;
            }
            map[y][x] = Square::Free;
        }
    }

    Ok(sum.to_string())
}

fn run_maze(file: &str) -> Result<HashSet<Pos>, Box<dyn Error>> {
    let mut guard_pos = Pos {
        y: 0,
        x: 0,
    };

    // 0 = up, 1 = right, 2 = down, 3 = left
    let mut direction = 0;

    let chars = vec!['.', '#', '^'];

    let map: Vec<Vec<Square>> = file
        .lines()
        .enumerate()
        .map(|(y, l)|
            l
                .chars()
                .filter(|c| chars.contains(c))
                .enumerate()
                .filter_map(|(x, e)| {
                    match e {
                        '#' => Some(Square::Box),
                        '^' => {
                            guard_pos.y = y.try_into().ok()?;
                            guard_pos.x = x.try_into().ok()?;
                            Some(Square::Free)
                        }
                        '.' => Some(Square::Free),
                        _ => None,
                    }
                })
                .collect()
        )
        .collect();

    let height = map.len().try_into()?;
    let width = map[0].len().try_into()?;

    let mut visited: HashSet<Pos> = HashSet::new();
    visited.insert(guard_pos);

    loop {
        let next_dir = DIRS[direction];
        let igy = guard_pos.y + next_dir.y;
        let igx = guard_pos.x + next_dir.x;

        if igy < 0 || igy >= height || igx < 0 || igx >= width {
            break;
        }

        let gy: usize = igy.try_into()?;
        let gx: usize = igx.try_into()?;

        let next_space = &map[gy][gx];

        match next_space {
            Square::Free => {
                guard_pos.x = gx.try_into()?;
                guard_pos.y = gy.try_into()?;
                visited.insert(guard_pos);
            }
            Square::Box => {
                direction = (direction + 1) % 4;
            }
        }
    }

    Ok(visited)
}

fn is_loop(map: &Vec<Vec<Square>>, guard: Pos) -> Result<(bool, usize), Box<dyn Error>> {
    let mut guard_pos = guard.clone();
    let mut direction = 0;

    let height = map.len().try_into()?;
    let width = map[0].len().try_into()?;

    let mut visited: HashSet<(Pos, usize)> = HashSet::new();
    visited.insert((guard_pos, direction));

    loop {
        let next_dir: Pos = DIRS[direction];
        let igy = guard_pos.y + next_dir.y;
        let igx = guard_pos.x + next_dir.x;

        if igy < 0 || igy >= height || igx < 0 || igx >= width {
            break;
        }

        let gy: usize = igy.try_into()?;
        let gx: usize = igx.try_into()?;

        let next_space = &map[gy][gx];

        match next_space {
            Square::Free => {
                guard_pos.x = gx.try_into()?;
                guard_pos.y = gy.try_into()?;
            }
            Square::Box => {
                direction = (direction + 1) % 4;
            }
        }

        if visited.contains(&(guard_pos, direction)) {
            return Ok((true, visited.len()));
        } else {
            visited.insert((guard_pos, direction));
            continue;
        }
    }

    Ok((false, visited.len()))
}
