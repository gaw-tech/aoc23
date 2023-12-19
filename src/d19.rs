use std::{collections::HashMap, fs};

pub fn solve() {
    let file_path = "input/day19.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut contents = contents.split("\n\n").filter(|e| e.len() > 0);

    let workflows = contents
        .next()
        .unwrap()
        .split("\n")
        .map(|e| {
            let n = e.find("{").unwrap();
            let name = &e[0..n];
            let rules = e[n + 1..e.len() - 1].split(",").collect::<Vec<&str>>();
            (name, rules)
        })
        .fold(HashMap::new(), |mut a, (n, r)| {
            a.insert(n, r);
            a
        });
    let parts = contents
        .next()
        .unwrap()
        .split("\n")
        .filter(|e| e.len() > 0)
        .map(|e| {
            let e = &e[1..e.len() - 1];
            let mut e = e.split(",");
            let x = e.next().unwrap()[2..].parse::<i64>().unwrap();
            let m = e.next().unwrap()[2..].parse::<i64>().unwrap();
            let a = e.next().unwrap()[2..].parse::<i64>().unwrap();
            let s = e.next().unwrap()[2..].parse::<i64>().unwrap();
            (x, m, a, s)
        })
        .collect::<Vec<(i64, i64, i64, i64)>>();

    let mut accepted = Vec::new();

    for part in &parts {
        let mut workflow = "in";
        loop {
            for rule in workflows.get(workflow).unwrap() {
                if rule.contains(":") {
                    let mut rule = rule.split(":");
                    let details = rule.next().unwrap();
                    let next_workflow = rule.next().unwrap();
                    let value = details[2..].parse::<i64>().unwrap();
                    let details = details.chars().collect::<Vec<char>>();
                    let part_value = {
                        match details[0] {
                            'x' => part.0,
                            'm' => part.1,
                            'a' => part.2,
                            's' => part.3,
                            _ => 0,
                        }
                    };
                    let passes = {
                        match details[1] {
                            '>' => part_value > value,
                            '<' => part_value < value,
                            _ => false,
                        }
                    };
                    if passes {
                        workflow = next_workflow;
                        break;
                    }
                } else {
                    workflow = *rule;
                    break;
                }
            }
            if workflow.eq("A") {
                accepted.push(part);
                break;
            }
            if workflow.eq("R") {
                break;
            }
        }
    }

    let res1 = accepted.iter().map(|e| e.0 + e.1 + e.2 + e.3).sum::<i64>();

    let mut queue = Vec::new();
    let mut accepted = Vec::new();

    queue.push(("in", (1, 4000), (1, 4000), (1, 4000), (1, 4000)));
    while !queue.is_empty() {
        let (workflow, mut x, mut m, mut a, mut s) = queue.pop().unwrap();
        if workflow.eq("R") {
            continue;
        }
        if workflow.eq("A") {
            accepted.push((x, m, a, s));
            continue;
        }
        for rule in workflows.get(workflow).unwrap() {
            if rule.contains(":") {
                let mut rule = rule.split(":");
                let details = rule.next().unwrap();
                let next_workflow = rule.next().unwrap();
                let value = details[2..].parse::<i64>().unwrap();
                let details = details.chars().collect::<Vec<char>>();
                match details[0] {
                    'x' => match details[1] {
                        '>' => {
                            if x.0 > value {
                                queue.push((next_workflow, x, m, a, s));
                                break;
                            } else if x.1 > value {
                                queue.push((next_workflow, (value + 1, x.1), m, a, s));
                                x.1 = value;
                            }
                        }
                        '<' => {
                            if x.1 < value {
                                queue.push((next_workflow, x, m, a, s));
                                break;
                            } else if x.0 < value {
                                queue.push((next_workflow, (x.0, value - 1), m, a, s));
                                x.0 = value;
                            }
                        }
                        _ => (),
                    },
                    'm' => match details[1] {
                        '>' => {
                            if m.0 > value {
                                queue.push((next_workflow, x, m, a, s));
                                break;
                            } else if m.1 > value {
                                queue.push((next_workflow, x, (value + 1, m.1), a, s));
                                m.1 = value;
                            }
                        }
                        '<' => {
                            if m.1 < value {
                                queue.push((next_workflow, x, m, a, s));
                                break;
                            } else if m.0 < value {
                                queue.push((next_workflow, x, (m.0, value - 1), a, s));
                                m.0 = value;
                            }
                        }
                        _ => (),
                    },
                    'a' => match details[1] {
                        '>' => {
                            if a.0 > value {
                                queue.push((next_workflow, x, m, a, s));
                                break;
                            } else if a.1 > value {
                                queue.push((next_workflow, x, m, (value + 1, a.1), s));
                                a.1 = value;
                            }
                        }
                        '<' => {
                            if a.1 < value {
                                queue.push((next_workflow, x, m, a, s));
                                break;
                            } else if a.0 < value {
                                queue.push((next_workflow, x, m, (a.0, value - 1), s));
                                a.0 = value;
                            }
                        }
                        _ => (),
                    },
                    's' => match details[1] {
                        '>' => {
                            if s.0 > value {
                                queue.push((next_workflow, x, m, a, s));
                                break;
                            } else if s.1 > value {
                                queue.push((next_workflow, x, m, a, (value + 1, s.1)));
                                s.1 = value;
                            }
                        }
                        '<' => {
                            if s.1 < value {
                                queue.push((next_workflow, x, m, a, s));
                                break;
                            } else if s.0 < value {
                                queue.push((next_workflow, x, m, a, (s.0, value - 1)));
                                s.0 = value;
                            }
                        }
                        _ => (),
                    },
                    _ => (),
                }
            } else {
                queue.push((rule, x, m, a, s));
            }
        }
    }
    
    let res2: i64 = accepted.iter().fold(0, |acc, (x, m, a, s)| {
        acc + (x.1 - x.0 + 1) * (m.1 - m.0 + 1) * (a.1 - a.0 + 1) * (s.1 - s.0 + 1)
    });

    println!("Day 19:\nTask 1:{res1:16}\nTask 2:{res2:16}");
}
