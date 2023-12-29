use std::{collections::HashMap, fmt::Debug};

use eval::{to_value, Expr};
use regex::Regex;

#[derive(Debug)]
struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}
impl Part {
    fn new(part: String) -> Part {
        let re = Regex::new(r"\{x=(?<x>\d+),m=(?<m>\d+),a=(?<a>\d+),s=(?<s>\d+)\}").unwrap();
        let capture = re.captures(part.as_str()).unwrap();
        Part {
            x: capture.name("x").unwrap().as_str().parse::<i64>().unwrap(),
            m: capture.name("m").unwrap().as_str().parse::<i64>().unwrap(),
            a: capture.name("a").unwrap().as_str().parse::<i64>().unwrap(),
            s: capture.name("s").unwrap().as_str().parse::<i64>().unwrap(),
        }
    }
}

#[derive(Debug)]
struct Workflow {
    conditions: Vec<String>,
    options: Vec<String>,
    default: String,
}

impl Workflow {
    fn new(conditions: String) -> Workflow {
        let mut result = Workflow {
            conditions: Vec::new(),
            options: Vec::new(),
            default: String::new(),
        };
        let (conditions, _) = conditions.split_once("}").unwrap();
        conditions.split(",").for_each(|e| {
            if e.contains(":") {
                let mut parts = e.split(":");
                result.conditions.push(String::from(
                    parts
                        .next()
                        .unwrap()
                        .split("<")
                        .collect::<Vec<_>>()
                        .join(" < ")
                        .split(">")
                        .collect::<Vec<_>>()
                        .join(" > "),
                ));
                result.options.push(String::from(parts.next().unwrap()));
            } else {
                result.default = String::from(e);
            }
        });
        result
    }
    fn evaluate(&self, value: &Part) -> String {
        for i in 0..self.conditions.len() {
            if Expr::new(self.conditions[i].clone())
                .value('s', value.s)
                .value('x', value.x)
                .value('m', value.m)
                .value('a', value.a)
                .exec()
                .unwrap()
                == to_value(true)
            {
                return self.options[i].clone();
            }
        }

        return self.default.clone();
    }
}

#[derive(Debug)]
struct Problem {
    workflows: HashMap<String, Workflow>,
    parts: Vec<Part>,
}

fn parse_input(input: &str) -> Problem {
    let mut parts = input.split("\n\n");
    let result = Problem {
        workflows: HashMap::from_iter(parts.next().unwrap().split("\n").map(|e| {
            let (name, rest) = e.split_once("{").unwrap();
            (String::from(name), Workflow::new(String::from(rest)))
        })),
        parts: parts
            .next()
            .unwrap()
            .split("\n")
            .map(|e| Part::new(String::from(e)))
            .collect(),
    };
    result
}

pub fn run_a(input: &str) {
    let problem = parse_input(input);
    let result = problem
        .parts
        .iter()
        .filter(|e| {
            let mut current_rule_name = String::from("in");
            while current_rule_name != "A" && current_rule_name != "R" {
                let current_rule = problem.workflows.get(&current_rule_name).unwrap();
                current_rule_name = current_rule.evaluate(e);
            }
            current_rule_name == "A"
        })
        .map(|e| e.x + e.m + e.a + e.s)
        .sum::<i64>();
    println!("Day 19a: {:?}", result);
}

fn extract_accepted_paths(
    workflows: &HashMap<String, Workflow>,
    current_rule_name: &str,
) -> Option<Vec<Vec<Condition>>> {
    if current_rule_name == "A" {
        return Some(vec![vec![]]);
    }
    if current_rule_name == "R" {
        return None;
    }

    let current_rule = workflows.get(current_rule_name).unwrap();
    let mut result = vec![];
    for i in 0..current_rule.conditions.len() {
        let paths = extract_accepted_paths(workflows, &current_rule.options[i]);
        if let Some(paths) = paths {
            paths.iter().for_each(|e| {
                let mut item = vec![Condition::new(&current_rule.conditions[i])];
                for j in 0..i {
                    item.push(Condition::new(&current_rule.conditions[j]).negate())
                }
                e.iter().for_each(|inner_e| item.push(inner_e.clone()));
                result.push(item);
            });
        }
    }
    let paths = extract_accepted_paths(workflows, &current_rule.default);
    if let Some(paths) = paths {
        paths.iter().for_each(|e| {
            let mut item: Vec<Condition> = current_rule
                .conditions
                .iter()
                .map(|e| Condition::new(e).negate())
                .collect::<Vec<_>>();
            e.iter().for_each(|inner_e| item.push(inner_e.clone()));
            result.push(item);
        });
    }
    return Some(result);
}
#[derive(Debug, Eq, PartialEq, Clone)]
enum OP {
    GT,
    LT,
}
#[derive(Clone)]
struct Condition {
    component: String,
    operator: OP,
    value: i64,
}
impl Condition {
    fn new(condition: &str) -> Condition {
        let mut parts = condition.split(" ");
        let component = String::from(parts.next().unwrap());
        let operator = match parts.next().unwrap() {
            "<" => OP::LT,
            ">" => OP::GT,
            _ => panic!("Unknown operator"),
        };
        let value = parts.next().unwrap().parse::<i64>().unwrap();
        Condition {
            component,
            operator,
            value,
        }
    }
    fn negate(&self) -> Condition {
        Condition {
            component: self.component.clone(),
            operator: match self.operator {
                OP::GT => OP::LT,
                OP::LT => OP::GT,
            },
            value: match self.operator {
                OP::GT => self.value + 1,
                OP::LT => self.value - 1,
            },
        }
    }
}

impl Debug for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.component,
            match self.operator {
                OP::GT => ">",
                OP::LT => "<",
            },
            self.value
        )
    }
}

fn simplify_bound(conditions: &Vec<Condition>, component: &str, operator: OP) -> Option<i64> {
    let result = conditions
        .iter()
        .filter(|e| e.component == component)
        .clone()
        .filter(|e| e.operator == operator)
        .map(|e| e.value);
    match operator {
        OP::GT => result.max(),
        OP::LT => result.min(),
    }
}

pub fn run_b(input: &str) {
    let problem = parse_input(input);
    let accepted_paths = extract_accepted_paths(&problem.workflows, "in")
        .unwrap()
        .iter()
        .map(|e| {
            let max_x = simplify_bound(&e, "x", OP::LT).unwrap_or(4001);
            let max_m = simplify_bound(&e, "m", OP::LT).unwrap_or(4001);
            let max_a = simplify_bound(&e, "a", OP::LT).unwrap_or(4001);
            let max_s = simplify_bound(&e, "s", OP::LT).unwrap_or(4001);
            let min_x = simplify_bound(&e, "x", OP::GT).unwrap_or(0);
            let min_m = simplify_bound(&e, "m", OP::GT).unwrap_or(0);
            let min_a = simplify_bound(&e, "a", OP::GT).unwrap_or(0);
            let min_s = simplify_bound(&e, "s", OP::GT).unwrap_or(0);
            (min_x, max_x, min_m, max_m, min_a, max_a, min_s, max_s)
        })
        .collect::<Vec<_>>();
    let mut borders = HashMap::new();
    borders.insert("x", vec![]);
    borders.insert("m", vec![]);
    borders.insert("a", vec![]);
    borders.insert("s", vec![]);
    accepted_paths
        .iter()
        .for_each(|(min_x, max_x, min_m, max_m, min_a, max_a, min_s, max_s)| {
            (*borders.get_mut("x").unwrap()).push(*min_x + 1);
            (*borders.get_mut("x").unwrap()).push(*max_x);

            (*borders.get_mut("m").unwrap()).push(*min_m + 1);
            (*borders.get_mut("m").unwrap()).push(*max_m);

            (*borders.get_mut("a").unwrap()).push(*min_a + 1);
            (*borders.get_mut("a").unwrap()).push(*max_a);

            (*borders.get_mut("s").unwrap()).push(*min_s + 1);
            (*borders.get_mut("s").unwrap()).push(*max_s);
        });
    borders.values_mut().for_each(|e| {
        let mut sorted = e.clone();
        sorted.sort();
        sorted.dedup();
        *e = sorted;
    });
    let mut result = 0;
    for x in borders.get("x").unwrap().windows(2) {
        let [x0, x1] = [x[0], x[1]];
        for m in borders.get("m").unwrap().windows(2) {
            let [m0, m1] = [m[0], m[1]];
            for a in borders.get("a").unwrap().windows(2) {
                let [a0, a1] = [a[0], a[1]];
                for s in borders.get("s").unwrap().windows(2) {
                    let [s0, s1] = [s[0], s[1]];
                    let part = Part {
                        x: x0,
                        m: m0,
                        a: a0,
                        s: s0,
                    };
                    if accepted_paths
                        .iter()
                        .find(|(min_x, max_x, min_m, max_m, min_a, max_a, min_s, max_s)| {
                            (*min_x < part.x && part.x < *max_x)
                                && (*min_m < part.m && part.m < *max_m)
                                && (*min_a < part.a && part.a < *max_a)
                                && (*min_s < part.s && part.s < *max_s)
                        })
                        .is_some()
                    {
                        result += (x1 - x0) * (m1 - m0) * (a1 - a0) * (s1 - s0);
                    }
                }
            }
        }
    }
    println!("");
    println!("Day 19b: {:?}", result);
}
pub fn run() {
    let challenge_input = include_str!("../inputs/day_19/challenge.txt");
    let _test_input = include_str!("../inputs/day_19/test.txt");
    run_a(challenge_input);
    run_b(challenge_input);
}

//173855935815500 -> 1 -> 4001
//167409079868000
