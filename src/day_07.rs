use std::{cmp::Ordering, collections::HashSet};

static CARDS_A: [&str; 13] = [
    "2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K", "A",
];
static CARDS_B: [&str; 13] = [
    "J", "2", "3", "4", "5", "6", "7", "8", "9", "T", "Q", "K", "A",
];
#[derive(Debug, Eq)]
struct Game {
    rank: i64,
    hand: Vec<i64>,
    bid: i64,
}

impl Ord for Game {
    fn cmp(&self, other: &Self) -> Ordering {
        let (rank_a, rank_b) = (self.rank, other.rank);
        if rank_a == rank_b {
            if let Some((score_a, score_b)) = self
                .hand
                .iter()
                .zip(other.hand.iter())
                .find(|(a, b)| a != b)
            {
                score_a.cmp(score_b)
            } else {
                Ordering::Equal
            }
        } else {
            rank_a.cmp(&rank_b)
        }
    }
}
impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.cmp(other).into()
    }
}
impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
impl Game {
    fn new(hand: Vec<i64>, bid: i64, has_joker: bool) -> Self {
        let rank = Game::rank(&hand, has_joker);
        Self { hand, bid, rank }
    }
    fn rank(hand: &Vec<i64>, has_joker: bool) -> i64 {
        let mut hand = hand;
        let mut joker_hand: Vec<i64> = hand.clone();
        if has_joker {
            let max_frequency = hand
                .iter()
                .fold([0; 13], |mut acc, e| {
                    if *e != 0 {
                        acc[*e as usize] += 1;
                    }
                    acc
                })
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.cmp(b))
                .unwrap()
                .0 as i64;

            joker_hand = joker_hand
                .iter()
                .map(|e| if *e == 0 { max_frequency } else { *e })
                .collect();
            hand = &joker_hand;
        }

        let unique: HashSet<&i64> = HashSet::from_iter(hand.iter());
        if unique.len() == 1 {
            6
        } else if unique.len() == 2 {
            let first_count = hand
                .iter()
                .filter(|e| e == unique.get(&hand[0]).unwrap())
                .count();
            if first_count == 4 || first_count == 1 {
                5
            } else {
                4
            }
        } else if unique.len() == 3 {
            let highest_count = unique
                .iter()
                .map(|u| hand.iter().filter(|e| e == u).count())
                .max()
                .unwrap();
            if highest_count == 3 {
                3
            } else {
                2
            }
        } else if unique.len() == 4 {
            1
        } else {
            0
        }
    }
}

fn parse_input(input: &str, cards: &[&str; 13], has_joker: bool) -> Vec<Game> {
    input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(" ").unwrap();
            let hand = hand
                .chars()
                .map(|card| cards.iter().position(|e| *e == String::from(card)).unwrap() as i64)
                .collect();
            Game::new(hand, bid.parse().unwrap(), has_joker)
        })
        .collect()
}

pub fn run_a(input: &str) {
    let mut result = parse_input(input, &CARDS_A, false);
    result.sort();
    let result = result
        .iter()
        .enumerate()
        .map(|(i, e)| ((i + 1) as i64) * e.bid)
        .sum::<i64>();
    println!("Day 07a: {:?}", result);
}

pub fn run_b(input: &str) {
    let mut result = parse_input(input, &CARDS_B, true);
    result.sort();
    let result = result
        .iter()
        .enumerate()
        .map(|(i, e)| ((i + 1) as i64) * e.bid)
        .sum::<i64>();
    println!("Day 07b: {:?}", result);
}

pub fn run() {
    let challenge_input = include_str!("../inputs/day_07/challenge.txt");
    let _test_input = include_str!("../inputs/day_07/test.txt");
    run_a(challenge_input);
    run_b(challenge_input);
}

// 249138943
