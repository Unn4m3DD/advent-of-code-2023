use std::collections::HashMap;

use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn parse_input(input: &str) -> (Vec<i64>, HashMap<(&str, &str), Vec<(i64, i64, i64)>>) {
    let mut input = input.split("\n\n");
    let seeds = input
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split(" ")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    let mut transformations: HashMap<(&str, &str), Vec<(i64, i64, i64)>> = HashMap::new();
    input.for_each(|map| {
        let mut lines = map.lines();
        let regex = regex::Regex::new(r"(?<from>[a-z]+)-to-(?<to>[a-z]+) map:").unwrap();
        let (from, to) = regex
            .captures(lines.next().unwrap())
            .map(|x| {
                (
                    x.name("from").unwrap().as_str(),
                    x.name("to").unwrap().as_str(),
                )
            })
            .unwrap();
        let transformation = lines
            .map(|e| {
                let mut split = e.split(" ");
                let to = split.next().unwrap().parse::<i64>().unwrap();
                let from = split.next().unwrap().parse::<i64>().unwrap();
                let amount = split.next().unwrap().parse::<i64>().unwrap();
                (from, to, amount)
            })
            .collect::<Vec<(i64, i64, i64)>>();
        transformations.insert((from, to), transformation);
    });
    (seeds, transformations)
}

const ORDER: [&str; 8] = [
    "seed",
    "soil",
    "fertilizer",
    "water",
    "light",
    "temperature",
    "humidity",
    "location",
];
fn explore_forwards(
    transformations: &HashMap<(&str, &str), Vec<(i64, i64, i64)>>,
    place: i64,
    place_index: i64,
) -> i64 {
    if place_index == ORDER.len() as i64 - 1 {
        return place;
    }
    if let Some(mapping) = transformations.get(&(
        ORDER[(place_index) as usize],
        ORDER[(place_index + 1) as usize],
    )) {
        if let Some((a, b, _)) = mapping
            .iter()
            .find(|(a, _, range)| *a <= place && place < *a + range)
        {
            return explore_forwards(transformations, place + *b - *a, place_index + 1);
        }
    }
    return explore_forwards(transformations, place, place_index + 1);
}

pub fn run_a(input: &str) {
    let (seeds, transformations) = parse_input(input);
    let mut result = i64::MAX;
    for seed in seeds {
        result = result.min(explore_forwards(&transformations, seed, 0 as i64))
    }
    println!("Day 05a: {:?}", result);
}

fn explore_backwards(
    transformations: &HashMap<(&str, &str), Vec<(i64, i64, i64)>>,
    place: i64,
    place_index: i64,
) -> i64 {
    if place_index == 0 {
        return place;
    }
    if let Some(mapping) = transformations.get(&(
        ORDER[(place_index - 1) as usize],
        ORDER[(place_index) as usize],
    )) {
        if let Some((a, b, _)) = mapping
            .iter()
            .find(|(_, b, range)| *b <= place && place < *b + range)
        {
            return explore_backwards(transformations, place + *a - *b, place_index - 1);
        }
    }
    return explore_backwards(transformations, place, place_index - 1);
}

pub fn run_b(input: &str) {
    let (seeds, transformations) = parse_input(input);
    let seeds = seeds
        .chunks(2)
        .map(|e| (e[0], e[1]))
        .collect::<Vec<(i64, i64)>>();

    let result = (0..=i64::MAX)
        .into_par_iter()
        .find_first(|i| {
            let origin = explore_backwards(&transformations, *i, (ORDER.len() - 1) as i64);
            seeds
                .iter()
                .find(|(a, range)| *a <= origin && origin < *a + range)
                .map_or(false, |_| true)
        })
        .unwrap();

    println!("Day 05b: {}", result);
}

pub fn run() {
    let challenge_input = include_str!("../inputs/day_05/challenge.txt");
    let _test_input = include_str!("../inputs/day_05/test.txt");
    run_a(challenge_input);
    run_b(challenge_input);
}
