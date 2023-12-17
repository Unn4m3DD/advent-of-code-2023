fn parse_input(input: &str) -> Vec<Vec<Vec<char>>> {
    input
        .split("\n\n")
        .map(|map| map.lines().map(|line| line.chars().collect()).collect())
        .collect()
}

fn get_vertical_reflection(input: &Vec<Vec<char>>) -> Option<i64> {
    'line_loop: for symmetry_line in 1..(input.len() as i64) {
        for j in 0..symmetry_line {
            for k in 0..input[0].len() {
                match (
                    input
                        .get((symmetry_line - j - 1) as usize)
                        .map_or(None, |e| Some(e[k])),
                    input
                        .get((symmetry_line + j) as usize)
                        .map_or(None, |e| Some(e[k])),
                ) {
                    (Some(a), Some(b)) => {
                        if a != b {
                            continue 'line_loop;
                        }
                    }
                    _ => {}
                }
            }
        }
        return Some(symmetry_line);
    }
    None
}
fn get_horizontal_reflection(input: &Vec<Vec<char>>) -> Option<i64> {
    'line_loop: for symmetry_line in 1..(input[0].len() as i64) {
        for j in 0..symmetry_line {
            for k in 0..input.len() {
                match (
                    input[k].get((symmetry_line - j - 1) as usize),
                    input[k].get((symmetry_line + j) as usize),
                ) {
                    (Some(a), Some(b)) => {
                        if a != b {
                            continue 'line_loop;
                        }
                    }
                    _ => {}
                }
            }
        }
        return Some(symmetry_line);
    }
    None
}

pub fn run_a(input: &str) {
    let result = parse_input(input);
    let result = result
        .iter()
        .map(|e| (get_vertical_reflection(e), get_horizontal_reflection(e)))
        .map(|(a, b)| a.unwrap_or(0) * 100 + b.unwrap_or(0))
        .sum::<i64>();

    println!("Day 13a: {}", result);
}

fn get_smudgy_vertical_reflection(input: &Vec<Vec<char>>) -> Option<i64> {
    'line_loop: for symmetry_line in 1..(input.len() as i64) {
        let mut counter = 0;
        for j in 0..symmetry_line {
            for k in 0..input[0].len() {
                match (
                    input
                        .get((symmetry_line - j - 1) as usize)
                        .map_or(None, |e| Some(e[k])),
                    input
                        .get((symmetry_line + j) as usize)
                        .map_or(None, |e| Some(e[k])),
                ) {
                    (Some(a), Some(b)) => {
                        if a != b {
                            if counter > 1 {
                                continue 'line_loop;
                            } else {
                                counter += 1;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        if counter == 1 {
            return Some(symmetry_line);
        }
    }
    None
}
fn get_smudgy_horizontal_reflection(input: &Vec<Vec<char>>) -> Option<i64> {
    'line_loop: for symmetry_line in 1..(input[0].len() as i64) {
        let mut counter = 0;
        for j in 0..symmetry_line {
            for k in 0..input.len() {
                match (
                    input[k].get((symmetry_line - j - 1) as usize),
                    input[k].get((symmetry_line + j) as usize),
                ) {
                    (Some(a), Some(b)) => {
                        if a != b {
                            if counter > 1 {
                                continue 'line_loop;
                            } else {
                                counter += 1;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        if counter == 1 {
            return Some(symmetry_line);
        }
    }
    None
}

pub fn run_b(input: &str) {
    let result = parse_input(input);
    let result = result
        .iter()
        .map(|e| {
            (
                get_smudgy_vertical_reflection(e),
                get_smudgy_horizontal_reflection(e),
            )
        })
        .map(|(a, b)| a.unwrap_or(0) * 100 + b.unwrap_or(0))
        .sum::<i64>();

    println!("Day 13b: {}", result);
}

pub fn run() {
    let challenge_input = include_str!("../inputs/day_13/challenge.txt");
    let _test_input = include_str!("../inputs/day_13/test.txt");
    run_a(challenge_input);
    run_b(challenge_input);
}
