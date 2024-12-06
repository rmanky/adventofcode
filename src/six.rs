use std::{ collections::HashSet, error::Error, fs };

pub fn six() -> Result<(), Box<dyn Error>> {
    println!("Day Six");

    let input = fs::read_to_string("inputs/six.txt")?;
    let (mut map, guard_pos) = parse_input(&input)?;

    let pt1 = run_maze(&map, guard_pos)?.len();
    println!("Part 1: {}", pt1);

    let pt2 = count_fillable_squares(map.as_mut(), guard_pos)?;
    println!("Part 2: {}", pt2);

    Ok(())
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Pos {
    y: i32,
    x: i32,
}

#[derive(Debug, PartialEq)]
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

fn parse_input(input: &str) -> Result<(Vec<Vec<Square>>, Pos), Box<dyn Error>> {
    let mut guard_pos = Pos { x: 0, y: 0 };
    let map = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(x, c)| {
                    match c {
                        '#' => Some(Square::Box),
                        '^' => {
                            guard_pos = Pos {
                                y: y as i32,
                                x: x as i32,
                            };
                            Some(Square::Free)
                        }
                        '.' => Some(Square::Free),
                        _ => None,
                    }
                })
                .collect()
        })
        .collect();
    Ok((map, guard_pos))
}

fn run_maze(map: &Vec<Vec<Square>>, mut guard_pos: Pos) -> Result<HashSet<Pos>, Box<dyn Error>> {
    let mut direction = 0;
    let height = map.len() as i32;
    let width = map[0].len() as i32;
    let mut visited = HashSet::new();
    visited.insert(guard_pos);

    loop {
        let next_dir = DIRS[direction];
        let igy = guard_pos.y + next_dir.y;
        let igx = guard_pos.x + next_dir.x;

        if igy < 0 || igy >= height || igx < 0 || igx >= width {
            break;
        }

        let gy = igy as usize;
        let gx = igx as usize;

        match map[gy][gx] {
            Square::Free => {
                guard_pos = Pos { x: gx as i32, y: gy as i32 };
                visited.insert(guard_pos);
            }
            Square::Box => {
                direction = (direction + 1) % 4;
            }
        }
    }

    Ok(visited)
}

fn count_fillable_squares(
    map: &mut Vec<Vec<Square>>,
    guard_pos: Pos
) -> Result<usize, Box<dyn Error>> {
    let candidates = run_maze(map, guard_pos)?;
    let mut sum = 0;

    for pos in candidates {
        let y = pos.y as usize;
        let x = pos.x as usize;
        if map[y][x] == Square::Free {
            map[y][x] = Square::Box;
            if is_loop(&map, guard_pos)? {
                sum += 1;
            }
            map[y][x] = Square::Free;
        }
    }

    Ok(sum)
}

fn is_loop(map: &Vec<Vec<Square>>, mut guard_pos: Pos) -> Result<bool, Box<dyn Error>> {
    let mut direction = 0;
    let height = map.len() as i32;
    let width = map[0].len() as i32;
    let mut visited = HashSet::new();
    visited.insert((guard_pos, direction));

    loop {
        let next_dir = DIRS[direction];
        let igy = guard_pos.y + next_dir.y;
        let igx = guard_pos.x + next_dir.x;

        if igy < 0 || igy >= height || igx < 0 || igx >= width {
            break;
        }

        let gy = igy as usize;
        let gx = igx as usize;

        match map[gy][gx] {
            Square::Free => {
                guard_pos = Pos { x: gx as i32, y: gy as i32 };
            }
            Square::Box => {
                direction = (direction + 1) % 4;
            }
        }

        if visited.contains(&(guard_pos, direction)) {
            return Ok(true);
        } else {
            visited.insert((guard_pos, direction));
        }
    }

    Ok(false)
}
