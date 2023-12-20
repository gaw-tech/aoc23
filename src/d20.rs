use std::{
    collections::{HashMap, VecDeque},
    fs,
};

use num::Integer;

pub fn solve() {
    let file_path = "input/day20.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents = contents
        .split("\n")
        .filter(|e| e.len() > 0)
        .map(|e| e.replace(" ", ""))
        .collect::<Vec<String>>();

    let mut modules = Vec::new();
    for i in 0..contents.len() {
        let e = &contents[i];
        let mut e = e.split("->");
        let module = e.next().unwrap();
        let receivers = e.next().unwrap().split(",").collect::<Vec<&str>>();
        let module_type = &module[0..1];
        let module_name = &module[1..];
        modules.push((module_name, module_type, receivers));
    }

    let n = modules.len();

    let mut module_map: HashMap<&str, Module<'_>> = HashMap::new();
    for i in 0..n {
        let (module_name, module_type, receives) = &modules[i];
        if module_type.eq(&"%") {
            module_map.insert(module_name, Module::Flipflop(false, receives.clone()));
        } else if module_type.eq(&"&") {
            let mut in_signals = Vec::new();
            for j in 0..n {
                if modules[j].2.contains(module_name) {
                    in_signals.push(modules[j].0.clone());
                }
            }
            let mut in_map = HashMap::new();
            for s in in_signals {
                in_map.insert(s, false);
            }
            module_map.insert(module_name, Module::Conjunction(in_map, receives.clone()));
        } else {
            module_map.insert(&"broadcaster", Module::Broadcaster(receives.clone()));
        }
    }

    let (mut l, mut h) = (0, 0);
    let module_map = &mut module_map;
    for _ in 0..1000 {
        let r = press(module_map);
        l += r.0;
        h += r.1;
    }

    let res1 = l * h;

    let target = "tj";
    let mut senders = Vec::new();
    for (k, m) in module_map.iter() {
        if let Module::Broadcaster(rec) = m {
            if rec.contains(&target) {
                senders.push(*k);
            }
        }
        if let Module::Conjunction(_, rec) = m {
            if rec.contains(&target) {
                senders.push(*k);
            }
        }
        if let Module::Flipflop(_, rec) = m {
            if rec.contains(&target) {
                senders.push(*k);
            }
        }
    }

    let mut presses_vec = Vec::new();
    for s in senders {
        let mut presses = 0;
        let contents =
            fs::read_to_string(file_path).expect("Should have been able to read the file");
        let contents = contents
            .split("\n")
            .filter(|e| e.len() > 0)
            .map(|e| e.replace(" ", ""))
            .collect::<Vec<String>>();

        let mut modules = Vec::new();
        for i in 0..contents.len() {
            let e = &contents[i];
            let mut e = e.split("->");
            let module = e.next().unwrap();
            let receivers = e.next().unwrap().split(",").collect::<Vec<&str>>();
            let module_type = &module[0..1];
            let module_name = &module[1..];
            modules.push((module_name, module_type, receivers));
        }

        let n = modules.len();

        let mut module_map: HashMap<&str, Module<'_>> = HashMap::new();
        for i in 0..n {
            let (module_name, module_type, receives) = &modules[i];
            if module_type.eq(&"%") {
                module_map.insert(module_name, Module::Flipflop(false, receives.clone()));
            } else if module_type.eq(&"&") {
                let mut in_signals = Vec::new();
                for j in 0..n {
                    if modules[j].2.contains(module_name) {
                        in_signals.push(modules[j].0.clone());
                    }
                }
                let mut in_map = HashMap::new();
                for s in in_signals {
                    in_map.insert(s, false);
                }
                module_map.insert(module_name, Module::Conjunction(in_map, receives.clone()));
            } else {
                module_map.insert(&"broadcaster", Module::Broadcaster(receives.clone()));
            }
        }

        loop {
            presses += 1;
            if press_and_end_on(&mut module_map, s) {
                presses_vec.push(presses);
                break;
            }
        }
    }

    let res2: i64 = presses_vec.iter().fold(1, |a, e| a.lcm(e));

    println!("Day 20:\nTask 1:{res1:16}\nTask 2:{res2:16}");
}

#[derive(Debug)]
enum Module<'a> {
    Flipflop(bool, Vec<&'a str>),
    Conjunction(HashMap<&'a str, bool>, Vec<&'a str>),
    Broadcaster(Vec<&'a str>),
}

fn press<'a, 'b>(module_map: &'a mut HashMap<&'b str, Module<'b>>) -> (i64, i64) {
    let mut low = 0;
    let mut high = 0;
    let mut queue = VecDeque::new();
    queue.push_back(("broadcaster", false, ""));
    while !queue.is_empty() {
        let (name, signal, source) = queue.pop_front().unwrap();
        if signal {
            high += 1;
        } else {
            low += 1;
        }
        let module = module_map.get_mut(&name);
        if let Some(Module::Broadcaster(rec)) = module {
            for s in rec {
                queue.push_back((s, signal, name));
            }
        } else if let Some(Module::Flipflop(state, rec)) = module {
            if !signal {
                let signal = *state;
                *state = !signal;
                for s in rec {
                    queue.push_back((s, *state, name))
                }
            }
        } else if let Some(Module::Conjunction(states, rec)) = module {
            states.insert(source, signal);
            let signal = !states.values().all(|e| *e);
            for s in rec {
                queue.push_back((s, signal, name));
            }
        }
    }
    (low, high)
}

fn press_and_end_on<'a, 'b>(module_map: &'a mut HashMap<&'b str, Module<'b>>, exit: &str) -> bool {
    let mut queue = VecDeque::new();
    queue.push_back(("broadcaster", false, ""));
    while !queue.is_empty() {
        let (name, signal, source) = queue.pop_front().unwrap();
        let module = module_map.get_mut(&name);
        if let Some(Module::Broadcaster(rec)) = module {
            for s in rec {
                queue.push_back((s, signal, name));
            }
        } else if let Some(Module::Flipflop(state, rec)) = module {
            if !signal {
                let signal = *state;
                *state = !signal;
                for s in rec {
                    queue.push_back((s, *state, name))
                }
            }
        } else if let Some(Module::Conjunction(states, rec)) = module {
            states.insert(source, signal);
            let signal = !states.values().all(|e| *e);
            if signal && name.eq(exit) {
                return true;
            }
            for s in rec {
                queue.push_back((s, signal, name));
            }
        }
    }
    false
}
