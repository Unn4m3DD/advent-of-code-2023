use std::collections::HashSet;

fn parse_input(input: &str) -> ((i64, i64), Vec<Vec<Vec<(i64, i64)>>>) {
    let mut s_pos: (i64, i64) = (0, 0);
    let grid = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, e)| match e {
                    '|' => vec![(0, 1), (0, -1)],
                    '-' => vec![(1, 0), (-1, 0)],
                    'L' => vec![(1, 0), (0, -1)],
                    'J' => vec![(-1, 0), (0, -1)],
                    '7' => vec![(-1, 0), (0, 1)],
                    'F' => vec![(1, 0), (0, 1)],
                    '.' => vec![],
                    'S' => {
                        s_pos = (x as i64, y as i64);
                        vec![(0, 1), (0, -1), (-1, 0), (1, 0)]
                    }
                    _ => panic!("Unknown char"),
                })
                .collect()
        })
        .collect();
    (s_pos, grid)
}

pub fn run_a(input: &str) {
    let (initial_pos, mut grid) = parse_input(input);
    // This doesn't work for S within the edges of the grid
    let right = grid[initial_pos.1 as usize][initial_pos.0 as usize + 1].clone();
    let left = grid[initial_pos.1 as usize][initial_pos.0 as usize - 1].clone();
    let up = grid[initial_pos.1 as usize - 1][initial_pos.0 as usize].clone();
    let down = grid[initial_pos.1 as usize + 1][initial_pos.0 as usize].clone();
    let mut s_dirs = vec![];
    if right.iter().find(|e| e == &&(-1, 0)).is_some() {
        s_dirs.push((1, 0));
    }
    if left.iter().find(|e| e == &&(1, 0)).is_some() {
        s_dirs.push((-1, 0));
    }
    if up.iter().find(|e| e == &&(0, 1)).is_some() {
        s_dirs.push((0, -1));
    }
    if down.iter().find(|e| e == &&(0, -1)).is_some() {
        s_dirs.push((0, 1));
    }
    let mut last_dir = s_dirs[0];
    grid[initial_pos.1 as usize][initial_pos.0 as usize] = s_dirs;

    let mut pipe_length = 0;
    let mut current_pos = initial_pos.clone();
    loop {
        let dir = grid[current_pos.1 as usize][current_pos.0 as usize]
            .iter()
            .find(|e| e != &&(-last_dir.0, -last_dir.1))
            .unwrap();
        last_dir = *dir;
        current_pos = (current_pos.0 + dir.0, current_pos.1 + dir.1);
        pipe_length += 1;
        if initial_pos == current_pos {
            break;
        }
    }

    println!("Day 10a: {}", pipe_length / 2);
}

fn _expand_reaches_out(
    grid: Vec<Vec<Vec<(i64, i64)>>>,
    start: (i64, i64),
    bounds: (i64, i64, i64, i64),
) -> bool {
    let dirs = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut visited = HashSet::new();
    let mut to_visit = vec![start];
    while let Some(current_pos) = to_visit.pop() {
        if current_pos.0 < bounds.0
            || current_pos.0 > bounds.2
            || current_pos.1 < bounds.1
            || current_pos.1 > bounds.3
        {
            return true;
        }
        if visited.contains(&current_pos) {
            continue;
        }
        visited.insert(current_pos);
        for dir in &dirs {
            if grid[dir.1 as usize][dir.0 as usize].len() == 0 {
                to_visit.push((current_pos.0 + dir.0, current_pos.1 + dir.1));
            }
        }
    }
    false
}

pub fn run_b(input: &str) {
    let (initial_pos, mut grid) = parse_input(input);
    // This doesn't work for S within the edges of the grid
    let right = grid[initial_pos.1 as usize][initial_pos.0 as usize + 1].clone();
    let left = grid[initial_pos.1 as usize][initial_pos.0 as usize - 1].clone();
    let up = grid[initial_pos.1 as usize - 1][initial_pos.0 as usize].clone();
    let down = grid[initial_pos.1 as usize + 1][initial_pos.0 as usize].clone();
    let mut s_dirs = vec![];
    if right.iter().find(|e| e == &&(-1, 0)).is_some() {
        s_dirs.push((1, 0));
    }
    if left.iter().find(|e| e == &&(1, 0)).is_some() {
        s_dirs.push((-1, 0));
    }
    if up.iter().find(|e| e == &&(0, 1)).is_some() {
        s_dirs.push((0, -1));
    }
    if down.iter().find(|e| e == &&(0, -1)).is_some() {
        s_dirs.push((0, 1));
    }
    let mut last_dir = s_dirs[0];
    grid[initial_pos.1 as usize][initial_pos.0 as usize] = s_dirs;

    let mut pipe = vec![];
    let mut current_pos = initial_pos.clone();
    loop {
        let dir = grid[current_pos.1 as usize][current_pos.0 as usize]
            .iter()
            .find(|e| e != &&(-last_dir.0, -last_dir.1))
            .unwrap();
        last_dir = *dir;
        pipe.push(current_pos);
        current_pos = (current_pos.0 + dir.0, current_pos.1 + dir.1);
        if initial_pos == current_pos {
            break;
        }
    }
    // for y in 0..grid.len() {
    //     for x in 0..grid[y].len() {
    //         if pipe.contains(&(x as i64, y as i64)) {
    //             print!("#");
    //         } else if grid[y][x].len() == 0 {
    //             print!(".");
    //         } else {
    //             print!(" ");
    //         }
    //     }
    //     println!();
    // }
    // println!();
    // let (min_x, min_y, max_x, max_y) = pipe.iter().fold((i64::MAX, i64::MAX, 0, 0), |acc, next| {
    //     (
    //         acc.0.min(next.0),
    //         acc.1.min(next.1),
    //         acc.2.max(next.0),
    //         acc.3.max(next.1),
    //     )
    // });
    // let mut contained = 0;
    // for y in min_y..=max_y {
    //     for x in min_x..=max_x {
    //         if grid[y as usize][x as usize].len() == 0 {
    //             if !expand_reaches_out(grid, (x, y), (min_x, min_y, max_x, max_y)) {
    //                 contained += 1;
    //             }
    //         }
    //     }
    // }
    // println!("Day 10b: {}", contained);
}

pub fn run() {
    let _challenge_input = include_str!("../inputs/day_10/challenge.txt");
    let _test_input = include_str!("../inputs/day_10/test.txt");
    run_a(_challenge_input);
    run_b(_challenge_input);
}
