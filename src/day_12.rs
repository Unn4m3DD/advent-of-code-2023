use std::collections::HashMap;

fn parse_input(input: &str) -> Vec<(&str, Vec<i64>)> {
    input
        .lines()
        .map(|line| {
            let result = line.split_once(" ").unwrap();
            (
                result.0,
                result
                    .1
                    .split(",")
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect(),
            )
        })
        .collect()
}

fn explore<'a>(
    springs: &'a [char],
    pattern: &Vec<i64>,
    memory: &mut HashMap<(&'a [char], Vec<i64>), i64>,
) -> i64 {
    if let Some(result) = memory.get(&(springs, pattern.to_vec())) {
        return *result;
    }
    let result;
    if springs.is_empty() && pattern.get(0).map_or(2, |e| *e) <= 0 && pattern.len() == 1 {
        result = 1;
    } else if springs.is_empty() || pattern.is_empty() {
        result = 0;
    } else if springs[0] == '#' && pattern[0] == 0 {
        result = 0;
    } else if springs[0] == '.' && pattern[0] > 0 {
        result = 0;
    } else if pattern[0] >= 0 {
        let mut filled_pattern = pattern.clone();
        filled_pattern[0] -= 1;
        result = explore(&springs[1..], &filled_pattern, memory)
    } else {
        if springs[0] == '?' {
            let mut shifted_pattern = pattern[1..].to_vec();
            result = explore(&springs[1..], &pattern, memory)
                + if !shifted_pattern.is_empty() {
                    shifted_pattern[0] -= 1;
                    explore(&springs[1..], &shifted_pattern, memory)
                } else {
                    0
                }
        } else if springs[0] == '#' {
            let mut shifted_pattern = pattern[1..].to_vec();
            if shifted_pattern.is_empty() {
                return 0;
            }
            shifted_pattern[0] -= 1;
            result = explore(&springs[1..], &shifted_pattern, memory)
        } else {
            result = explore(&springs[1..], &pattern, memory)
        }
    }
    memory.insert((springs, pattern.to_vec()), result);
    result
}

pub fn run_a(input: &str) {
    let result = parse_input(input);
    let result = result
        .iter()
        .map(|e| {
            let mut memory = HashMap::new();
            explore(
                &e.0.chars().collect::<Vec<_>>(),
                &vec![-1].iter().chain(&e.1).map(|e| *e).collect::<Vec<_>>(),
                &mut memory,
            )
        })
        .collect::<Vec<_>>();

    println!("Day 12a: {:?}", result.iter().sum::<i64>());
}

pub fn run_b(input: &str) {
    let result = parse_input(input);
    let result = result
        .iter()
        .map(|e| (vec![e.0].repeat(5).join("?"), e.1.repeat(5)))
        .map(|e| {
            let mut memory = HashMap::new();
            explore(
                &e.0.chars().collect::<Vec<_>>(),
                &vec![-1].iter().chain(&e.1).map(|e| *e).collect::<Vec<_>>(),
                &mut memory,
            )
        })
        .collect::<Vec<_>>();

    println!("Day 12b: {:?}", result.iter().sum::<i64>());
}

pub fn run() {
    let challenge_input = include_str!("../inputs/day_12/challenge.txt");
    let _test_input = include_str!("../inputs/day_12/test.txt");
    run_a(challenge_input);
    run_b(challenge_input);
}
