use std::collections::HashSet;

pub fn parse_input(input: &str) -> (Vec<Vec<&str>>, (i64, i64)) {
    let mut start_pos = (0, 0);
    let grid = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '.' => ".",
                    '#' => "#",
                    'S' => {
                        start_pos = (x as i64, y as i64);
                        "."
                    }
                    _ => panic!("Unknown char {}", c),
                })
                .collect()
        })
        .collect();
    (grid, start_pos)
}

pub fn run_a(input: &str) {
    let remaining_steps = 64;
    let (grid, start_pos) = parse_input(input);
    let mut reached = HashSet::new();
    let mut visited = HashSet::new();
    let mut to_visit = vec![(start_pos, remaining_steps)];
    while let Some((pos, remaining_steps)) = to_visit.pop() {
        if remaining_steps == 0 {
            reached.insert(pos);
            continue;
        }
        if visited.contains(&(pos, remaining_steps)) {
            continue;
        }
        visited.insert((pos, remaining_steps));
        for (dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let new_pos = (pos.0 + dx, pos.1 + dy);
            if let Some(&".") = grid
                .get(new_pos.1 as usize)
                .and_then(|row| row.get(new_pos.0 as usize))
            {
                to_visit.push((new_pos, remaining_steps - 1));
            }
        }
    }
    println!("Day 21a: {}", reached.len());
}

pub fn _run_b(input: &str) {
    let (grid, start_pos) = parse_input(input);
    let remaining_steps = 30;
    let mut reached = HashSet::new();
    let mut visited = HashSet::new();
    let mut to_visit = vec![(start_pos, remaining_steps)];
    let len0 = grid[0].len() as i64;
    let len1 = grid.len() as i64;
    while let Some((pos, remaining_steps)) = to_visit.pop() {
        if remaining_steps == 0 {
            reached.insert(pos);
            continue;
        }
        if visited.contains(&(pos, remaining_steps)) {
            continue;
        }
        visited.insert((pos, remaining_steps));
        for (dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let new_pos = ((pos.0 + dx), (pos.1 + dy));

            if let Some(&".") = grid
                .get(((new_pos.1 % len1 + len1) % len1) as usize)
                .and_then(|row| row.get(((new_pos.0 % len0 + len0) % len0) as usize))
            {
                to_visit.push((new_pos, remaining_steps - 1));
            }
        }
    }
    // There is probably some strategy around multiplying the inner fully reached patterns and summing then rounding the outer edges 
    let min_x = *reached.iter().map(|(x, _)| x).min().unwrap();
    let min_y = *reached.iter().map(|(y, _)| y).min().unwrap();
    let max_x = *reached.iter().map(|(x, _)| x).max().unwrap();
    let max_y = *reached.iter().map(|(y, _)| y).max().unwrap();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if (x + 1) % len1 == 0 {
                print!(" ");
            }
            if reached.contains(&(x, y)) {
                print!("O");
            } else if let Some(&".") = grid
                .get(((y % len1 + len1) % len1) as usize)
                .and_then(|row| row.get(((x % len0 + len0) % len0) as usize))
            {
                print!(".");
            } else {
                print!("#");
            }
        }

        if (y + 1) % len0 == 0 {
            println!("");
        }
        println!();
    }
    println!();
    println!("Day 21b: {}", reached.len());
}

pub fn run() {
    let challenge_input = include_str!("../inputs/day_21/challenge.txt");
    let _test_input = include_str!("../inputs/day_21/test.txt");
    run_a(challenge_input);
    // run_b(test_input);
}
