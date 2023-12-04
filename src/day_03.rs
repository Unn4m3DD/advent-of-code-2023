fn input_to_vec(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn read_part_number(input: &mut Vec<Vec<char>>, i: usize, j: usize) -> Option<i64> {
    let mut result = String::new();
    let mut j = j;
    let mut has_symbol = false;
    while input
        .get(i)
        .unwrap()
        .get(j)
        .map_or(false, |e| e.is_digit(10))
    {
        result += &input[i][j].to_string();
        input[i][j] = '.';
        for x in i as i64 - 1..=i as i64 + 1 {
            for y in j as i64 - 1..=j as i64 + 1 {
                if input.get(x as usize).map_or(false, |e| {
                    e.get(y as usize)
                        .map_or(false, |e| !e.is_digit(10) && *e != '.')
                }) {
                    has_symbol = true;
                }
            }
        }
        j += 1;
    }
    if result.is_empty() || !has_symbol {
        None
    } else {
        Some(result.parse::<i64>().unwrap())
    }
}

fn get_ratio(input: &mut Vec<Vec<char>>, i: i64, j: i64) -> Option<i64> {
    if input[i as usize][j as usize] != '*' {
        return None;
    }
    let mut result = 1;
    let mut to_visit = vec![
        (i - 1, j - 1),
        (i, j - 1),
        (i + 1, j - 1),
        (i - 1, j),
        (i + 1, j),
        (i - 1, j + 1),
        (i, j + 1),
        (i + 1, j + 1),
    ];
    let mut found = 0;
    while let Some((x, y)) = to_visit.pop() {
        if input.get(x as usize).map_or(false, |e| {
            e.get(y as usize).map_or(false, |e| e.is_digit(10))
        }) {
            let mut number_start = (x, y);
            while input.get(number_start.0 as usize).map_or(false, |e| {
                e.get(number_start.1 as usize)
                    .map_or(false, |e| e.is_digit(10))
            }) {
                number_start.1 -= 1;
            }
            number_start.1 += 1;
            let mut number = String::new();

            while let Some(digit) = input
                .get(number_start.0 as usize)
                .map_or(None, |e| e.get(number_start.1 as usize))
            {
                if !digit.is_digit(10) {
                    break;
                }
                number += String::from(*digit).as_str();
                if let Some(index) = to_visit
                    .iter()
                    .position(|e| e.0 == number_start.0 && e.1 == number_start.1)
                {
                    to_visit.swap_remove(index);
                }
                number_start.1 += 1;
            }
            if let Ok(number) = number.parse::<i64>() {
                if number > 0 {
                    result *= number;
                    found += 1;
                }
            };
        }
    }
    if found == 2 {
        Some(result)
    } else {
        None
    }
}

pub fn run_a(input: &str) {
    let mut result = input_to_vec(input);
    let mut total = 0;
    for i in 0..result.len() {
        for j in 0..result[i].len() {
            if let Some(number) = read_part_number(&mut result, i, j) {
                total += number;
            }
        }
    }
    println!("Day 03a: {}", total);
}
pub fn run_b(input: &str) {
    let mut result = input_to_vec(input);
    let mut total = 0;
    for i in 0..result.len() {
        for j in 0..result[i].len() {
            if let Some(number) = get_ratio(&mut result, i as i64, j as i64) {
                total += number;
            }
        }
    }
    println!("Day 03b: {}", total);
}

pub fn run() {
    let challenge_input = include_str!("../inputs/day_03/challenge.txt");
    let _test_input = include_str!("../inputs/day_03/test.txt");
    run_a(challenge_input);
    run_b(challenge_input);
}

