fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn manhattan_distance(a: (usize, usize), b: (usize, usize)) -> usize {
    ((a.0 as isize - b.0 as isize).abs() + (a.1 as isize - b.1 as isize).abs()) as usize
}

pub fn run_a(input: &str) {
    let mut grid = parse_input(input);
    let mut to_duplicate_rows = vec![];
    for row in 0..grid.len() {
        if grid[row].iter().all(|&x| x == '.') {
            to_duplicate_rows.push(row);
        }
    }
    let mut to_duplicate_cols = vec![];
    for col in 0..grid[0].len() {
        let mut all_empty = true;
        for row in 0..grid.len() {
            if grid[row][col] != '.' {
                all_empty = false;
                break;
            }
        }
        if all_empty {
            to_duplicate_cols.push(col);
        }
    }
    let mut offset = 0;
    for row in to_duplicate_rows {
        grid.insert(row + offset, vec!['.'; grid[0].len()]);
        offset += 1
    }
    let mut offset = 0;
    for col in to_duplicate_cols.iter() {
        for row in 0..grid.len() {
            grid[row].insert(col + offset, '.');
        }
        offset += 1
    }

    let mut galaxies = vec![];
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == '#' {
                galaxies.push((row, col));
            }
        }
    }
    let mut result = 0;
    for g1_idx in 0..galaxies.len() {
        for g2_idx in g1_idx..galaxies.len() {
            let g1 = galaxies[g1_idx];
            let g2 = galaxies[g2_idx];
            result += manhattan_distance(g1, g2);
        }
    }
    println!("Day 11a: {}", result);
}
pub fn run_b(input: &str) {
    let grid = parse_input(input);
    let mut galaxies = vec![];
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == '#' {
                galaxies.push((row, col));
            }
        }
    }
    let mut to_duplicate_rows = vec![];
    for row in 0..grid.len() {
        if grid[row].iter().all(|&x| x == '.') {
            to_duplicate_rows.push(row);
        }
    }
    let mut to_duplicate_cols = vec![];
    for col in 0..grid[0].len() {
        let mut all_empty = true;
        for row in 0..grid.len() {
            if grid[row][col] != '.' {
                all_empty = false;
                break;
            }
        }
        if all_empty {
            to_duplicate_cols.push(col);
        }
    }
    let mut result: u64 = 0;
    for g1_idx in 0..galaxies.len() {
        for g2_idx in g1_idx..galaxies.len() {
            let g1 = galaxies[g1_idx];
            let g2 = galaxies[g2_idx];
            let mut dist = 0;
            let mut range = [g1.0, g2.0];
            range.sort();
            for row in range[0]..range[1] {
                dist += if to_duplicate_rows.contains(&row) {
                    1000000
                } else {
                    1
                };
            }

            let mut range = [g1.1, g2.1];
            range.sort();
            for col in range[0]..range[1] {
                dist += if to_duplicate_cols.contains(&col) {
                    1000000
                } else {
                    1
                };
            }
            result += dist;
        }
    }
    println!("Day 11b: {}", result);
}

pub fn run() {
    let challenge_input = include_str!("../inputs/day_11/challenge.txt");
    let _test_input = include_str!("../inputs/day_11/test.txt");
    run_a(challenge_input);
    run_b(challenge_input);
}
