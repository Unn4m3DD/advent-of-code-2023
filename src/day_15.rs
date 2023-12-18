use regex::Regex;

fn parse_input(input: &str) -> Vec<&str> {
    input.split(",").collect()
}

fn hash(input: &&str) -> usize {
    let mut current_value = 0;
    for c in input.chars() {
        current_value = ((current_value + c as usize) * 17) % 256;
    }
    current_value
}

pub fn run_a(input: &str) {
    let result = parse_input(input);
    let result = result.iter().map(hash).sum::<usize>();
    println!("Day 15a: {}", result);
}
pub fn run_b(input: &str) {
    let equals_regex = Regex::new(r"(?<name>\w+)=(?<intensity>\d+)").unwrap();
    let minus_regex = Regex::new(r"(?<name>\w+)-").unwrap();
    let input = parse_input(input);
    let mut boxes = vec![vec![]; 256];
    for instruction in input {
        if let Some(caps) = equals_regex.captures(instruction) {
            let name = caps.name("name").unwrap().as_str();
            let intensity = caps
                .name("intensity")
                .unwrap()
                .as_str()
                .parse::<i64>()
                .unwrap();
            if let Some(idx) = boxes[hash(&name)].iter().position(|(n, _)| n == &name) {
                boxes[hash(&name)][idx] = (name, intensity);
            } else {
                boxes[hash(&name)].push((name, intensity));
            }
        }
        if let Some(caps) = minus_regex.captures(instruction) {
            let name = caps.name("name").unwrap().as_str();

            if let Some(idx) = boxes[hash(&name)].iter().position(|(n, _)| n == &name) {
                boxes[hash(&name)].remove(idx);
            }
        }
    }
    let result = boxes
        .iter()
        .enumerate()
        .map(|(i, b)| {
            ((i + 1) as i64)
                * b.iter()
                    .enumerate()
                    .map(|(i, (_, f))| ((i + 1) as i64) * f)
                    .sum::<i64>()
        })
        .sum::<i64>();
    println!("Day 15b: {}", result);
}

pub fn run() {
    let challenge_input = include_str!("../inputs/day_15/challenge.txt");
    let _test_input = include_str!("../inputs/day_15/test.txt");
    run_a(challenge_input);
    run_b(challenge_input);
}
