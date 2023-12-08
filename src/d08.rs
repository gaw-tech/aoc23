use std::{collections::HashMap, fs};

use num::Integer;

pub fn solve() {
    let file_path = "input/day08.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut contents = contents.split("\n\n").filter(|e| e.len() > 0);
    let path: Vec<char> = contents.next().unwrap().chars().collect();
    let map = contents
        .next()
        .unwrap()
        .split("\n")
        .filter(|e| e.len() > 0)
        .map(|e| {(e[0..3].to_string(), (e[7..10].to_string(), e[12..15].to_string()))})
        .fold(HashMap::new(), |mut a, (n, p)| {
            a.insert(n, p);
            a
        });
    let mut pos = "AAA".to_string();
    let mut path_index = 0;
    let mut res1 = 0;

    while pos != "ZZZ" {
        let cur = map.get(&pos).unwrap();
        let dir = path.get(path_index).unwrap();
        if *dir == 'L' {
            pos = cur.0.clone();
        } else {
            pos = cur.1.clone();
        }
        res1 += 1;
        path_index = (path_index + 1) % path.len();
    }

    let cycles: Vec<i64> = map
        .keys()
        .into_iter()
        .filter(|e| e.ends_with("A"))
        .map(|e| e.clone())
        .map(|mut pos| {
            let mut path_index = 0;
            let mut steps = 0;
            while !pos.ends_with("Z") {
                let cur = map.get(&pos).unwrap();
                let dir = path.get(path_index).unwrap();
                if *dir == 'L' {
                    pos = cur.0.clone();
                } else {
                    pos = cur.1.clone();
                }
                steps += 1;
                path_index = (path_index + 1) % path.len();
            }
            steps
        })
        .collect();

    let res2 = cycles.iter().fold(1, |a, e| a.lcm(e));

    println!("Day 08:\nTask 1:{res1:16}\nTask 2:{res2:16}");
}
