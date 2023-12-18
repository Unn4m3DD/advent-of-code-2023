use std::collections::HashSet;

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Beam {
    position: Point,
    direction: Point,
}

fn energized_amount(grid: &Vec<Vec<char>>, initial_beam: Beam) -> i64 {
    let mut visited_with_dir = HashSet::new();
    let mut visited = HashSet::new();
    let mut beams = vec![initial_beam];
    while let Some(beam) = beams.pop() {
        let new_position = Point {
            x: beam.position.x + beam.direction.x,
            y: beam.position.y + beam.direction.y,
        };
        if visited_with_dir.contains(&(new_position, beam.direction)) {
            continue;
        }
        match grid
            .get(new_position.y as usize)
            .map_or(None, |e| e.get(new_position.x as usize))
        {
            Some('|') => {
                visited_with_dir.insert((new_position, beam.direction));
                visited.insert(new_position);
                if beam.direction.x != 0 {
                    beams.push(Beam {
                        position: new_position,
                        direction: Point { x: 0, y: -1 },
                    });
                    beams.push(Beam {
                        position: new_position,
                        direction: Point { x: 0, y: 1 },
                    });
                } else {
                    beams.push(Beam {
                        position: new_position,
                        direction: beam.direction,
                    });
                }
            }
            Some('-') => {
                visited_with_dir.insert((new_position, beam.direction));
                visited.insert(new_position);
                if beam.direction.y != 0 {
                    beams.push(Beam {
                        position: new_position,
                        direction: Point { x: -1, y: 0 },
                    });
                    beams.push(Beam {
                        position: new_position,
                        direction: Point { x: 1, y: 0 },
                    });
                } else {
                    beams.push(Beam {
                        position: new_position,
                        direction: beam.direction,
                    });
                }
            }
            Some('/') => {
                visited_with_dir.insert((new_position, beam.direction));
                visited.insert(new_position);
                let new_direction = match beam.direction {
                    Point { x: -1, y: _ } => Point { x: 0, y: 1 },
                    Point { x: 1, y: _ } => Point { x: 0, y: -1 },
                    Point { x: _, y: -1 } => Point { x: 1, y: 0 },
                    Point { x: _, y: 1 } => Point { x: -1, y: 0 },
                    _ => Point { x: 0, y: 0 },
                };
                beams.push(Beam {
                    position: new_position,
                    direction: new_direction,
                });
            }
            Some('\\') => {
                visited_with_dir.insert((new_position, beam.direction));
                visited.insert(new_position);
                let new_direction = match beam.direction {
                    Point { x: -1, y: _ } => Point { x: 0, y: -1 },
                    Point { x: 1, y: _ } => Point { x: 0, y: 1 },
                    Point { x: _, y: -1 } => Point { x: -1, y: 0 },
                    Point { x: _, y: 1 } => Point { x: 1, y: 0 },
                    _ => Point { x: 0, y: 0 },
                };
                beams.push(Beam {
                    position: new_position,
                    direction: new_direction,
                });
            }
            Some('.') => {
                visited_with_dir.insert((new_position, beam.direction));
                visited.insert(new_position);
                beams.push(Beam {
                    position: new_position,
                    direction: beam.direction,
                });
            }
            _ => {}
        }
    }
    visited.len() as i64
}

pub fn run_a(input: &str) {
    let grid: Vec<Vec<char>> = parse_input(input);
    let result = energized_amount(
        &grid,
        Beam {
            position: Point { x: -1, y: 0 },
            direction: Point { x: 1, y: 0 },
        },
    );
    println!("Day 16a: {}", result);
}
pub fn run_b(input: &str) {
    let grid = parse_input(input);
    let mut result = (0..grid.len())
        .map(|e| {
            energized_amount(
                &grid,
                Beam {
                    position: Point { x: -1, y: e as i64 },
                    direction: Point { x: 1, y: 0 },
                },
            )
            .max(energized_amount(
                &grid,
                Beam {
                    position: Point {
                        x: grid[0].len() as i64,
                        y: e as i64,
                    },
                    direction: Point { x: -1, y: 0 },
                },
            ))
        })
        .max()
        .unwrap();
    result = result.max(
        (0..grid[0].len())
            .map(|e| {
                energized_amount(
                    &grid,
                    Beam {
                        position: Point { x: e as i64, y: -1 },
                        direction: Point { x: 0, y: 1 },
                    },
                )
                .max(energized_amount(
                    &grid,
                    Beam {
                        position: Point {
                            x: e as i64,
                            y: grid.len() as i64,
                        },
                        direction: Point { x: 0, y: -1 },
                    },
                ))
            })
            .max()
            .unwrap(),
    );
    println!("Day 16b: {}", result);
}

pub fn run() {
    let challenge_input = include_str!("../inputs/day_16/challenge.txt");
    let _test_input = include_str!("../inputs/day_16/test.txt");
    run_a(challenge_input);
    run_b(challenge_input);
}
