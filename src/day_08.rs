use std::collections::HashMap;

enum LR {
    L,
    R,
}

fn parse_input(input: &str) -> (Vec<LR>, HashMap<&str, (&str, &str)>) {
    let (lr, rest) = input.split_once("\n\n").unwrap();
    let mut graph = HashMap::new();
    let regex = regex::Regex::new(r"(?<source>\w+) = \((?<left>\w+), (?<right>\w+)\)").unwrap();
    rest.lines()
        .map(|e| regex.captures(e).unwrap())
        .for_each(|e| {
            let source = e.name("source").unwrap().as_str();
            let left = e.name("left").unwrap().as_str();
            let right = e.name("right").unwrap().as_str();
            graph.insert(source, (left, right));
        });
    (
        lr.chars()
            .map(|c| match c {
                'L' => LR::L,
                'R' => LR::R,
                _ => panic!("Unknown character"),
            })
            .collect(),
        graph,
    )
}

pub fn run_a(input: &str) {
    let (instructions, graph) = parse_input(input);
    let mut current_node = "AAA";
    let mut steps = 0;
    'outer: loop {
        for instruction in instructions.iter() {
            let (left, right) = graph.get(current_node).unwrap();
            match instruction {
                LR::L => current_node = left,
                LR::R => current_node = right,
            }
            steps += 1;
            if current_node == "ZZZ" {
                break 'outer;
            }
        }
    }
    println!("Day 08a: {}", steps);
}

fn gcd(a: i64, b: i64) -> i64 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a.abs()
}

fn lcm(numbers: &Vec<i64>) -> i64 {
    let mut numbers = numbers.clone();
    numbers.sort();
    let mut lcm = numbers[0];
    for &number in numbers.iter().skip(1) {
        lcm = (lcm * number) / gcd(lcm, number);
    }
    lcm
}

pub fn run_b(input: &str) {
    let (instructions, graph) = parse_input(input);
    let current_nodes = graph
        .keys()
        .filter(|e| e.ends_with("A"))
        .collect::<Vec<_>>();
    let steps = current_nodes
        .iter()
        .map(|&&current_node| {
            let mut current_node = current_node;
            let mut steps = 0;
            'outer: loop {
                for instruction in instructions.iter() {
                    let (left, right) = graph.get(current_node).unwrap();
                    match instruction {
                        LR::L => current_node = left,
                        LR::R => current_node = right,
                    }
                    steps += 1;
                    if current_node.ends_with("Z") {
                        break 'outer;
                    }
                }
            }
            steps
        })
        .collect::<Vec<_>>();

    println!("Day 08b: {}", lcm(&steps));
}

pub fn run() {
    let challenge_input = include_str!("../inputs/day_08/challenge.txt");
    let _test_input = include_str!("../inputs/day_08/test.txt");
    run_a(challenge_input);
    run_b(challenge_input);
}
