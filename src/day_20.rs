use std::collections::BTreeMap;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
enum Module {
    FF(bool),
    Conjunction(BTreeMap<String, bool>),
    Broadcaster,
    Output,
}
#[derive(Debug, Clone)]
struct Problem {
    modules: BTreeMap<String, Module>,
    connections: BTreeMap<String, Vec<String>>,
}
impl Problem {
    fn new(from: &str) -> Self {
        let mut result = Problem {
            modules: BTreeMap::new(),
            connections: BTreeMap::new(),
        };
        from.lines().for_each(|e| {
            let (module, targets) = e.split_once(" -> ").unwrap();
            let connections = targets.trim().split(", ").map(|e| e.to_string()).collect();
            if module.contains("&") {
                result.modules.insert(
                    module[1..].to_string(),
                    Module::Conjunction(BTreeMap::new()),
                );
                result
                    .connections
                    .insert(module[1..].to_string(), connections);
            } else if module.contains("%") {
                result
                    .modules
                    .insert(module[1..].to_string(), Module::FF(false));
                result
                    .connections
                    .insert(module[1..].to_string(), connections);
            } else {
                result
                    .modules
                    .insert(module.to_string(), Module::Broadcaster);
                result.connections.insert(module.to_string(), connections);
            }
        });

        result.modules.insert("output".to_string(), Module::Output);
        result.connections.insert("output".to_string(), vec![]);
        result.modules.insert("rx".to_string(), Module::Output);
        result.connections.insert("rx".to_string(), vec![]);
        for (module, targets) in result.connections.iter() {
            for target in targets {
                match result.modules.get_mut(target) {
                    Some(Module::Conjunction(map)) => {
                        map.insert(module.clone(), false);
                    }
                    _ => {}
                }
            }
        }
        result
    }

    fn propagate(&mut self, new_pulse: bool, has_rx_been_pulsed: &mut bool) -> (i64, i64) {
        let mut pulse_count = (0, 0);
        let mut to_propagate = vec![("button".to_string(), "broadcaster".to_string(), new_pulse)];
        while let Some((source, name, pulse)) = to_propagate.pop() {
            pulse_count = (
                pulse_count.0 + if pulse { 0 } else { 1 },
                pulse_count.1 + if pulse { 1 } else { 0 },
            );
            let module = self.modules.get_mut(&name).unwrap();
            match module {
                Module::FF(state) => {
                    if !pulse {
                        *state = !*state;
                        for target in self.connections.get(&name).unwrap() {
                            to_propagate.push((name.clone(), target.clone(), *state));
                        }
                    }
                }
                Module::Conjunction(map) => {
                    *map.get_mut(&source).unwrap() = pulse;
                    let to_sent_pulse = map.values().all(|e| *e);
                    for target in self.connections.get(&name).unwrap() {
                        to_propagate.push((name.clone(), target.clone(), !to_sent_pulse));
                    }
                }
                Module::Broadcaster => {
                    for target in self.connections.get(&name).unwrap() {
                        to_propagate.push((name.clone(), target.clone(), pulse));
                    }
                }
                _ => {
                    if !pulse {
                        *has_rx_been_pulsed = true;
                    }
                }
            }
        }
        pulse_count
    }
}

pub fn run_a(input: &str) {
    let mut problem = Problem::new(input);
    let mut result = (0, 0);
    for _ in 0..1000 {
        let propagation_result = problem.propagate(false, &mut false);
        result = (
            result.0 + propagation_result.0,
            result.1 + propagation_result.1,
        );
    }
    println!("Day 20a: {}", result.0 * result.1);
}
pub fn run_b(_input: &str) {
    // let mut problem = Problem::new(input);
    // let mut has_rx_been_pulsed = false;
    // let mut count = 0;
    // let mut memory = HashSet::new();
    // while !has_rx_been_pulsed {
    //     problem.propagate(false, &mut has_rx_been_pulsed);
    //     if let Some(state) = memory.get(&problem.modules) {
    //         println!("repeating {:#?}", state);
    //         return;
    //     } else {
    //         memory.insert(problem.modules.clone());
    //     }
    //     if count % 100000 == 0 {
    //         println!("{}", count);
    //     }
    //     count += 1;
    // }
    // println!("Day 20b: {}", count);
}

pub fn run() {
    let challenge_input = include_str!("../inputs/day_20/challenge.txt");
    let _test_input = include_str!("../inputs/day_20/test.txt");
    run_a(challenge_input);
    run_b(challenge_input);
}
