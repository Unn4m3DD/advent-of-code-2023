use std::collections::HashMap;

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn swap(grid: &mut Vec<Vec<char>>, source: (usize, usize), destination: (usize, usize)) {
    let tmp = grid[source.0][source.1];
    grid[source.0][source.1] = grid[destination.0][destination.1];
    grid[destination.0][destination.1] = tmp;
}

fn roll_north(mut grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    for i in 1..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'O' {
                let mut boulder_destination = (i - 1) as i64;
                while boulder_destination != -1 && grid[boulder_destination as usize][j] == '.' {
                    boulder_destination -= 1;
                }
                swap(&mut grid, (i, j), ((boulder_destination + 1) as usize, j));
            }
        }
    }
    grid
}

fn roll_south(mut grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    for i in (0..grid.len() - 1).rev() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'O' {
                let mut boulder_destination = (i + 1) as i64;
                while boulder_destination != grid.len() as i64
                    && grid[boulder_destination as usize][j] == '.'
                {
                    boulder_destination += 1;
                }
                swap(&mut grid, (i, j), ((boulder_destination - 1) as usize, j));
            }
        }
    }
    grid
}

fn roll_west(mut grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    for j in 1..grid[0].len() {
        for i in 0..grid.len() {
            if grid[i][j] == 'O' {
                let mut boulder_destination = (j - 1) as i64;
                while boulder_destination != -1 as i64
                    && grid[i][boulder_destination as usize] == '.'
                {
                    boulder_destination -= 1;
                }
                swap(&mut grid, (i, j), (i, (boulder_destination + 1) as usize));
            }
        }
    }
    grid
}
fn roll_east(mut grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    for j in (0..grid[0].len() - 1).rev() {
        for i in 0..grid.len() {
            if grid[i][j] == 'O' {
                let mut boulder_destination = (j + 1) as i64;
                while boulder_destination != grid[0].len() as i64
                    && grid[i][boulder_destination as usize] == '.'
                {
                    boulder_destination += 1;
                }
                swap(&mut grid, (i, j), (i, (boulder_destination - 1) as usize));
            }
        }
    }
    grid
}

fn count_load(grid: Vec<Vec<char>>) -> usize {
    let height = grid.len() as i64;
    grid.iter()
        .enumerate()
        .map(|(i, row)| row.iter().filter(|&&c| c == 'O').count() * (height - i as i64) as usize)
        .sum()
}

pub fn run_a(input: &str) {
    let input = parse_input(input);
    let rolled = roll_north(input);
    let result = count_load(rolled);
    println!("Day 14a: {}", result);
}
pub fn run_b(input: &str) {
    let mut memory: HashMap<Vec<Vec<char>>, i64> = HashMap::new();
    let mut next = parse_input(input);
    let mut i = 0;
    while i < 1000000000 {
        next = roll_north(next);
        next = roll_west(next);
        next = roll_south(next);
        next = roll_east(next);
        if let Some(old) = memory.get(&next) {
            let diff = i - old;
            i += diff * ((1000000000 - i) / diff);
            memory.clear();
        } else {
            memory.insert(next.clone(), i);
        }
        i += 1;
    }
    let result = count_load(next);
    println!("Day 14b: {}", result);
}

pub fn run() {
    let challenge_input = include_str!("../inputs/day_14/challenge.txt");
    let _test_input = include_str!("../inputs/day_14/test.txt");
    run_a(challenge_input);
    run_b(challenge_input);
}
