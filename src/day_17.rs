use std::collections::HashMap;

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|e| e.chars().map(|c| c.to_string().parse().unwrap()).collect())
        .collect()
}

const DIRS: [Point; 4] = [
    Point { x: 1, y: 0 },
    Point { x: 0, y: -1 },
    Point { x: 0, y: 1 },
    Point { x: -1, y: 0 },
];

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct SearchCase {
    position: Point,
    last_direction: Point,
    last_direction_repetitions: i64,
    path: Vec<Point>,
    cost: i64,
}
impl SearchCase {
    fn new(
        position: Point,
        last_direction: Point,
        last_direction_repetitions: i64,
        path: Vec<Point>,
        grid: &Vec<Vec<i64>>,
    ) -> SearchCase {
        let cost = if path.len() > 0 {
            path.iter()
                .map(|e| grid[e.y as usize][e.x as usize])
                .sum::<i64>()
        } else {
            i64::MAX
        };
        SearchCase {
            position,
            last_direction,
            last_direction_repetitions,
            path,
            cost,
        }
    }
}
impl Ord for SearchCase {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(&other)
            .unwrap_or(std::cmp::Ordering::Equal)
    }
}
impl PartialOrd for SearchCase {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}
impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.x + self.y).partial_cmp(&(other.x + other.y))
    }
}

pub fn run_a(input: &str) {
    let grid = parse_input(input);
    let destination = Point {
        x: grid[0].len() as i64 - 1,
        y: grid.len() as i64 - 1,
    };
    let mut memory = HashMap::new();
    let mut to_search = vec![SearchCase::new(
        Point { x: 0, y: 0 },
        Point { x: 0, y: 0 },
        0,
        vec![],
        &grid,
    )];
    let mut result = SearchCase::new(Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, 0, vec![], &grid);
    while let Some(case) = to_search.pop() {
        if let Some(cost) = memory.get(&(
            case.position,
            case.last_direction,
            case.last_direction_repetitions,
        )) {
            if case.cost >= *cost {
                continue;
            }
        }
        memory.insert(
            (
                case.position,
                case.last_direction,
                case.last_direction_repetitions,
            ),
            case.cost,
        );
        if case.position == destination {
            if result.cost > case.cost {
                result = case.clone();
            }
            continue;
        }
        for dir in &DIRS {
            let new_pos = Point {
                x: case.position.x + dir.x,
                y: case.position.y + dir.y,
            };
            if new_pos.x < 0
                || new_pos.y < 0
                || new_pos.x >= grid[0].len() as i64
                || new_pos.y >= grid.len() as i64
            {
                continue;
            }
            let mut last_direction_repetitions = case.last_direction_repetitions + 1;
            if case.last_direction != *dir {
                last_direction_repetitions = 1;
            } else if last_direction_repetitions > 3 {
                continue;
            }
            if case.last_direction.x == 0 && dir.x == 0 && case.last_direction.y == -dir.y {
                continue;
            }
            if case.last_direction.y == 0 && dir.y == 0 && case.last_direction.x == -dir.x {
                continue;
            }
            if case.path.contains(&new_pos) {
                continue;
            }
            let mut new_path = case.path.clone();
            new_path.push(new_pos);
            let new_case =
                SearchCase::new(new_pos, *dir, last_direction_repetitions, new_path, &grid);
            let index = to_search.binary_search(&new_case).unwrap_or_else(|e| e);
            to_search.insert(index, new_case);
        }
    }
    println!("Day 17a: {}", result.cost);
}
pub fn run_b(input: &str) {
    let grid = parse_input(input);
    let destination = Point {
        x: grid[0].len() as i64 - 1,
        y: grid.len() as i64 - 1,
    };
    let mut memory = HashMap::new();
    let mut to_search = vec![
        SearchCase::new(
            Point { x: 1, y: 0 },
            Point { x: 1, y: 0 },
            1,
            vec![Point { x: 1, y: 0 }],
            &grid,
        ),
        SearchCase::new(
            Point { x: 0, y: 1 },
            Point { x: 0, y: 1 },
            1,
            vec![Point { x: 0, y: 1 }],
            &grid,
        ),
    ];
    let mut result = SearchCase::new(Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, 0, vec![], &grid);
    while let Some(case) = to_search.pop() {
        if let Some(cost) = memory.get(&(
            case.position,
            case.last_direction,
            case.last_direction_repetitions,
        )) {
            if case.cost >= *cost {
                continue;
            }
        }
        memory.insert(
            (
                case.position,
                case.last_direction,
                case.last_direction_repetitions,
            ),
            case.cost,
        );
        if case.position == destination && 3 < case.last_direction_repetitions {
            if result.cost > case.cost {
                result = case.clone();
            }
            continue;
        }
        for dir in &DIRS {
            let new_pos = Point {
                x: case.position.x + dir.x,
                y: case.position.y + dir.y,
            };
            if new_pos.x < 0
                || new_pos.y < 0
                || new_pos.x >= grid[0].len() as i64
                || new_pos.y >= grid.len() as i64
            {
                continue;
            }
            let mut last_direction_repetitions = case.last_direction_repetitions + 1;
            if last_direction_repetitions <= 4 {
                if case.last_direction != *dir {
                    continue;
                }
            }
            if last_direction_repetitions > 10 {
                if case.last_direction == *dir {
                    continue;
                }
            }
            if case.last_direction != *dir {
                last_direction_repetitions = 1;
            }

            if case.last_direction.x == 0 && dir.x == 0 && case.last_direction.y == -dir.y {
                continue;
            }
            if case.last_direction.y == 0 && dir.y == 0 && case.last_direction.x == -dir.x {
                continue;
            }
            if case.path.contains(&new_pos) {
                continue;
            }
            let mut new_path = case.path.clone();
            new_path.push(new_pos);
            let new_case =
                SearchCase::new(new_pos, *dir, last_direction_repetitions, new_path, &grid);
            let index = to_search.binary_search(&new_case).unwrap_or_else(|e| e);
            to_search.insert(index, new_case);
        }
    }
    println!("Day 17b: {}", result.cost);
}

pub fn run() {
    let challenge_input = include_str!("../inputs/day_17/challenge.txt");
    let _test_input = include_str!("../inputs/day_17/test.txt");
    run_a(challenge_input);
    run_b(challenge_input);
}
