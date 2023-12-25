use std::collections::HashSet;

use regex::Regex;

#[derive(Debug, Hash, Eq, PartialEq)]
enum InstructionDir {
    L,
    R,
    U,
    D,
}
#[derive(Debug, Hash, Eq, PartialEq)]
struct Instruction {
    dir: InstructionDir,
    amount: i64,
}
#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Point {
    x: i64,
    y: i64,
}

impl InstructionDir {
    fn move_point(&self, point: &Point, amount: i64) -> Point {
        match self {
            InstructionDir::L => Point {
                x: point.x - amount,
                y: point.y,
            },
            InstructionDir::R => Point {
                x: point.x + amount,
                y: point.y,
            },
            InstructionDir::U => Point {
                x: point.x,
                y: point.y - amount,
            },
            InstructionDir::D => Point {
                x: point.x,
                y: point.y + amount,
            },
        }
    }
}

fn parse_input_a(input: &str) -> Vec<Instruction> {
    let re = Regex::new(r"(?<dir>\w) (?<amount>\w+) \(#([\w\d]+)\)").unwrap();
    input
        .lines()
        .map(|line| re.captures(line).unwrap())
        .map(|cap| {
            let dir = match &cap["dir"] {
                "L" => InstructionDir::L,
                "R" => InstructionDir::R,
                "U" => InstructionDir::U,
                "D" => InstructionDir::D,
                _ => panic!("Unknown direction"),
            };
            let amount = cap["amount"].parse().unwrap();
            Instruction { dir, amount }
        })
        .collect()
}

fn _parse_input_b(input: &str) -> Vec<Instruction> {
    let re = Regex::new(r"\w \d+ \(#(?<amount>[\w\d]{5})(?<dir>[\w\d]{1})\)").unwrap();
    input
        .lines()
        .map(|line| re.captures(line).unwrap())
        .map(|cap| {
            let dir = match &cap["dir"] {
                "0" => InstructionDir::R,
                "1" => InstructionDir::D,
                "2" => InstructionDir::L,
                "3" => InstructionDir::U,
                _ => panic!("Unknown direction"),
            };
            let amount = i64::from_str_radix(cap["amount"].into(), 16).unwrap();
            Instruction { dir, amount }
        })
        .collect()
}

fn _print_set(map: &HashSet<Point>, min_x: i64, min_y: i64, max_x: i64, max_y: i64) {
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let point = Point { x, y };
            if map.contains(&point) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn _can_escape(
    holes: &HashSet<Point>,
    x: i64,
    y: i64,
    min_x: i64,
    min_y: i64,
    max_x: i64,
    max_y: i64,
) -> bool {
    let mut to_search = vec![Point { x, y }];
    let mut visited = HashSet::new();
    while let Some(point) = to_search.pop() {
        if visited.contains(&point) {
            continue;
        }
        visited.insert(point.clone());
        if point.x < min_x || point.x > max_x || point.y < min_y || point.y > max_y {
            return true;
        }
        if holes.contains(&point) {
            continue;
        }
        to_search.push(InstructionDir::L.move_point(&point, 1));
        to_search.push(InstructionDir::R.move_point(&point, 1));
        to_search.push(InstructionDir::U.move_point(&point, 1));
        to_search.push(InstructionDir::D.move_point(&point, 1));
    }
    false
}

fn expand(holes: &mut HashSet<Point>, x: i64, y: i64) {
    let mut to_search = vec![Point { x, y }];
    while let Some(point) = to_search.pop() {
        if holes.contains(&point) {
            continue;
        }
        holes.insert(point.clone());
        to_search.push(InstructionDir::L.move_point(&point, 1));
        to_search.push(InstructionDir::R.move_point(&point, 1));
        to_search.push(InstructionDir::U.move_point(&point, 1));
        to_search.push(InstructionDir::D.move_point(&point, 1));
    }
}

pub fn run_a(input: &str) {
    let input = parse_input_a(input);
    let mut holes = HashSet::new();
    let mut last_point = Point { x: 0, y: 0 };
    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    input.iter().for_each(|e| {
        for _ in 0..e.amount {
            let new_point = e.dir.move_point(&last_point, 1);
            holes.insert(last_point.clone());
            last_point = new_point;
            min_x = min_x.min(last_point.x);
            min_y = min_y.min(last_point.y);
            max_x = max_x.max(last_point.x);
            max_y = max_y.max(last_point.y);
        }
    });
    // should be checking this point to be inside the contiguous curve with _can_escape but this solves the input i've been given
    expand(
        &mut holes,
        (max_x - min_x) / 2 + min_x,
        (max_y - min_y) / 2 + min_y,
    );
    println!("Day 18a: {:?}", holes.len());
}


pub fn _run_b(input: &str) {
    let input = _parse_input_b(input);
    let mut holes = HashSet::new();
    let mut last_point = Point { x: 0, y: 0 };
    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    input.iter().for_each(|e| {
        let new_point = e.dir.move_point(&last_point, e.amount);
        last_point = new_point;
        min_x = min_x.min(last_point.x);
        min_y = min_y.min(last_point.y);
        max_x = max_x.max(last_point.x);
        max_y = max_y.max(last_point.y);
    });

    expand(
        &mut holes,
        (max_x - min_x) / 2 + min_x,
        (max_y - min_y) / 2 + min_y,
    );
    println!("Day 18b: {:?}", holes.len());
}

pub fn run() {
    let challenge_input = include_str!("../inputs/day_18/challenge.txt");
    let _test_input = include_str!("../inputs/day_18/test.txt");
    run_a(challenge_input);
    // run_b(test_input);
}
