use std::{fs, collections::HashMap};

pub fn solve() {
    let file_path = "input/day12.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents: Vec<(&str, Vec<i64>)> = contents
        .split("\n")
        .filter(|e| e.len() > 0)
        .map(|e| {
            let mut e = e.split(" ");
            let record = e.next().unwrap();
            let groups = e
                .next()
                .unwrap()
                .split(",")
                .map(|e| e.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            (record, groups)
        })
        .collect();

    let res1 = contents.iter().fold(0, |a, e| a + arrangements(&e.0.chars().collect(), &e.1));
    let res2 = contents.iter().map(|(r, g)| {
        let r = r.to_string() + "?" + r + "?" + r + "?" + r + "?" + r;
        let g = g.clone();
        let mut og = g.clone();
        og.append(&mut g.clone());
        og.append(&mut g.clone());
        og.append(&mut g.clone());
        og.append(&mut g.clone());
        (r,og)
    }).fold(0, |a, e| a + arrangements2(&e.0.chars().collect(), &e.1, &mut HashMap::new()));

    println!("Day 12:\nTask 1:{res1:16}\nTask 2:{res2:16}");
}

fn arrangements(record: &Vec<char>, groups: &Vec<i64>) -> i64 {
    //println!("{:?}\n{:?}\n", record, groups);
    if groups.is_empty() {
        if !record.contains(&'#') {
            return 1;
        } else {
            return 0;
        }
    }
    if record.is_empty() {
        return 0;
    }
    let first = record.first().unwrap();
    let record = record[1..].to_vec();
    match first {
        '.' => arrangements(&record, groups),
        '#' => {
            let n = (groups.first().unwrap() - 1) as usize;
            let groups = groups[1..].to_vec();
            if let Some(part) = record.get(0..n) {
                let record = record[n..].to_vec();
                if !part.contains(&'.') {
                    if let Some(c) = record.first() {
                        if *c != '#' {
                            arrangements(&record[1..].to_vec(), &groups)
                        } else {
                            0
                        }
                    } else {
                        arrangements(&record, &groups)
                    }
                } else {
                    0
                }
            } else {
                0
            }
        }
        '?' => {
            let alt_arr = arrangements(&record, groups);
            let n = (groups.first().unwrap() - 1) as usize;
            let groups = groups[1..].to_vec();
            if let Some(part) = record.get(0..n) {
                let record = record[n..].to_vec();
                if !part.contains(&'.') {
                    if let Some(c) = record.first() {
                        if *c != '#' {
                            arrangements(&record[1..].to_vec(), &groups) + alt_arr
                        } else {
                            alt_arr
                        }
                    } else {
                        arrangements(&record, &groups) + alt_arr
                    }
                } else {
                    alt_arr
                }
            } else {
                alt_arr
            }
        }
        _ => panic!("Bad char in record!")
    }
}

fn arrangements2(record: &Vec<char>, groups: &Vec<i64>, map: &mut HashMap<Vec<char>, HashMap<Vec<i64>, i64>>) -> i64 {
    if groups.is_empty() {
        if !record.contains(&'#') {
            return 1;
        } else {
            return 0;
        }
    }
    if record.is_empty() {
        return 0;
    }
    if let Some(map) = map.get(record) {
        if let Some(val) = map.get(groups) {
            return *val;
        }
    }
    let or = record;
    let og = groups;

    let res;

    let first = record.first().unwrap();
    let record = record[1..].to_vec();
    match first {
        '.' => res = arrangements2(&record, groups, map),
        '#' => {
            let n = (groups.first().unwrap() - 1) as usize;
            let groups = groups[1..].to_vec();
            if let Some(part) = record.get(0..n) {
                let record = record[n..].to_vec();
                if !part.contains(&'.') {
                    if let Some(c) = record.first() {
                        if *c != '#' {
                            res = arrangements2(&record[1..].to_vec(), &groups, map);
                        } else {
                            res = 0;
                        }
                    } else {
                        res = arrangements2(&record, &groups, map);
                    }
                } else {
                    res = 0;
                }
            } else {
                res = 0;
            }
        }
        '?' => {
            let alt_arr = arrangements2(&record, groups, map);
            let n = (groups.first().unwrap() - 1) as usize;
            let groups = groups[1..].to_vec();
            if let Some(part) = record.get(0..n) {
                let record = record[n..].to_vec();
                if !part.contains(&'.') {
                    if let Some(c) = record.first() {
                        if *c != '#' {
                            res = arrangements2(&record[1..].to_vec(), &groups, map) + alt_arr;
                        } else {
                            res = alt_arr;
                        }
                    } else {
                        res = arrangements2(&record, &groups, map) + alt_arr;
                    }
                } else {
                    res = alt_arr;
                }
            } else {
                res = alt_arr;
            }
        }
        _ => panic!("Bad char in record!")
    }
    if let Some(new_map) = map.get(or) {
        let mut new_map = new_map.clone();
        new_map.insert(og.clone(), res);
        map.insert(or.clone(), new_map.clone());
    } else {
        let mut new_map = HashMap::new();
        new_map.insert(og.clone(), res);
        map.insert(or.clone(), new_map);
    }
    res
}