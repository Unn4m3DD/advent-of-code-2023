fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| line.split(" ").map(|e| e.parse::<i64>().unwrap()).collect())
        .collect()
}

fn extrapolate(seq: &Vec<i64>) -> i64 {
    let mut buffer = vec![];
    buffer.push((*seq).clone());
    while !buffer.last().unwrap().iter().all(|e| e == &0) {
        let diffs = buffer
            .last()
            .unwrap()
            .windows(2)
            .map(|w| w[1] - w[0])
            .collect::<Vec<i64>>();
        buffer.push(diffs);
    }
    let buffer_len = buffer.len();
    let tmp = *buffer[buffer_len - 1].last().unwrap();
    buffer[buffer_len - 1].push(tmp);
    for i in (0..buffer_len - 2).rev() {
        let extrapolated = *buffer[i].last().unwrap() + buffer[i + 1].last().unwrap();
        buffer[i].push(extrapolated);
    }
    *buffer[0].last().unwrap()
}

pub fn run_a(input: &str) {
    let input = parse_input(input);
    let result = input.iter().map(|e| extrapolate(e)).sum::<i64>();
    println!("Day 09a: {}", result);
}
pub fn run_b(input: &str) {
    let mut input = parse_input(input);
    let result = input
        .iter_mut()
        .map(|e| {
            e.reverse();
            e
        })
        .map(|e| extrapolate(e))
        .sum::<i64>();
    println!("Day 09b: {}", result);
}

pub fn run() {
    let challenge_input = include_str!("../inputs/day_09/challenge.txt");
    let _test_input = include_str!("../inputs/day_09/test.txt");
    run_a(challenge_input);
    run_b(challenge_input);
}
