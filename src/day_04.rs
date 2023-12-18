fn score(line: &str) -> i64 {
    line.split_once(":").unwrap().1;
    let (key, value) = line.split_once(" | ").unwrap();
    let (key, value) = (
        key.split(" ")
            .map(|x| x.parse::<i32>().unwrap_or(-1))
            .collect::<Vec<i32>>(),
        value
            .split(" ")
            .map(|x| x.parse::<i32>().unwrap_or(-2))
            .collect::<Vec<i32>>(),
    );
    let match_count = key.iter().filter(|x| value.contains(x)).count() as u32;
    if match_count == 0 {
        0
    } else {
        (2 as i64).pow(match_count - 1)
    }
}
fn count(line: &str) -> i64 {
    line.split_once(":").unwrap().1;
    let (key, value) = line.split_once(" | ").unwrap();
    let (key, value) = (
        key.split(" ")
            .map(|x| x.parse::<i32>().unwrap_or(-1))
            .collect::<Vec<i32>>(),
        value
            .split(" ")
            .map(|x| x.parse::<i32>().unwrap_or(-2))
            .collect::<Vec<i32>>(),
    );
    key.iter().filter(|x| value.contains(x)).count() as i64
}

pub fn run_a(input: &str) {
    let cards: i64 = input.lines().map(score).sum();
    println!("Day 04a: {:?}", cards);
}
pub fn run_b(input: &str) {
    let cards = input.lines().map(count).collect::<Vec<i64>>();
    let mut result = cards.clone().iter().map(|_| 1).collect::<Vec<i64>>();
    for i in 0..cards.len() {
        for _ in 0..result[i] {
            for j in i + 1..=(i + cards[i] as usize) {
                if let None = result.get(j) {
                    continue;
                }
                result[j] += 1;
            }
        }
    }
    println!("Day 04b: {:?}", result.iter().sum::<i64>());
}

pub fn run() {
    let challenge_input = include_str!("../inputs/day_04/challenge.txt");
    let _test_input = include_str!("../inputs/day_04/test.txt");
    run_a(challenge_input);
    run_b(challenge_input);
}
