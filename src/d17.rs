use std::{collections::HashMap, fs};

pub fn solve() {
    let file_path = "input/day17.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let map: Vec<Vec<i32>> = contents
        .split("\n")
        .filter(|e| e.len() > 0)
        .map(|e| e.chars().map(|e| e.to_digit(10).unwrap() as i32).collect())
        .collect();

    let res1 = find_path(&map, 1, 3);
    let res2 = find_path(&map, 4, 10);

    println!("Day 17:\nTask 1:{res1:16}\nTask 2:{res2:16}");
}

fn find_path(map: &Vec<Vec<i32>>, min: usize, max: usize) -> i32 {
    let n = map.len();
    assert!(map.iter().all(|e| e.len() == n));

    let mut queue: Vec<(i32, usize, usize, char)> = Vec::new();
    let mut table = HashMap::new();

    table.insert((0, 0, 'r'), 0);
    queue.push((0, 0, 0, 'r'));
    queue.push((0, 0, 0, 'd'));
    loop {
        queue.sort();
        queue.dedup();
        let (cur_val, x, y, d) = queue.remove(0);

        if (x, y) == (n - 1, n - 1) {
            return cur_val;
        }

        if d == 'r' || d == 'l' {
            // up
            let mut next_val = cur_val;
            for i in 1..min {
                if y.wrapping_sub(i) < n {
                    next_val += map[y - i][x];
                }
            }
            for i in min..=max {
                if y.wrapping_sub(i) < n {
                    let key = (x, y - i, 'u');
                    next_val += map[y - i][x];
                    let next = *table.get(&key).unwrap_or(&i32::MAX);
                    if next > next_val {
                        table.insert(key, next_val);
                        queue.push((next_val, x, y - i, 'u'))
                    }
                }
            }
            // down
            let mut next_val = cur_val;
            for i in 1..min {
                if y + i < n {
                    next_val += map[y + i][x];
                }
            }
            for i in min..=max {
                if y + i < n {
                    let key = (x, y + i, 'd');
                    next_val += map[y + i][x];
                    let next = *table.get(&key).unwrap_or(&i32::MAX);
                    if next > next_val {
                        table.insert(key, next_val);
                        queue.push((next_val, x, y + i, 'd'))
                    }
                }
            }
        }
        if d == 'u' || d == 'd' {
            // left
            let mut next_val = cur_val;
            for i in 1..min {
                if x.wrapping_sub(i) < n {
                    next_val += map[y][x - i];
                }
            }
            for i in min..=max {
                if x.wrapping_sub(i) < n {
                    let key = (x - i, y, 'l');
                    next_val += map[y][x - i];
                    let next = *table.get(&key).unwrap_or(&i32::MAX);
                    if next > next_val {
                        table.insert(key, next_val);
                        queue.push((next_val, x - i, y, 'l'))
                    }
                }
            }
            // right
            let mut next_val = cur_val;
            for i in 1..min {
                if x + i < n {
                    next_val += map[y][x + i];
                }
            }
            for i in min..=max {
                if x + i < n {
                    let key = (x + i, y, 'r');
                    next_val += map[y][x + i];
                    let next = *table.get(&key).unwrap_or(&i32::MAX);
                    if next > next_val {
                        table.insert(key, next_val);
                        queue.push((next_val, x + i, y, 'r'))
                    }
                }
            }
        }
    }
}
