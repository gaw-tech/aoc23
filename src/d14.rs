use std::{collections::HashMap, fs};

pub fn solve() {
    let file_path = "input/day14.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut map = contents 
        .split("\n")
        .filter(|e| e.len() > 0)
        .map(|e| e.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let n = map.len();
    let mut table = vec![0; n];
    for y in 0..n {
        for x in 0..n {
            let c = map[y][x];
            if c == '#' {
                table[x] = y + 1;
            } else if c == 'O' {
                map[y][x] = '.';
                map[table[x]][x] = 'O';
                table[x] += 1;
            }
        }
    }

    let res1 = map.iter().enumerate().fold(0, |a, (i, e)| {
        e.iter().filter(|e| **e == 'O').count() * (n - i) + a
    });

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents = contents
        .split("\n")
        .filter(|e| e.len() > 0)
        .map(|e| e.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let mut map = contents;
    let mut hm = HashMap::new();
    for i in 0..1000 {
        let v = cycle(&mut map);
        if let Some(old) = hm.insert(v, i) {
            let c = i - old;
            for _ in 0..((1_000_000_000 - i) % c - 1) {
                cycle(&mut map);
            }
            break;
        }
    }

    let res2 = map.iter().enumerate().fold(0, |a, (i, e)| {
        e.iter().filter(|e| **e == 'O').count() * (n - i) + a
    });

    println!("Day 14:\nTask 1:{res1:16}\nTask 2:{res2:16}");
}

fn cycle(map: &mut Vec<Vec<char>>) -> i64 {
    let n = map.len();

    // north
    let mut table = vec![0; n];
    for y in 0..n {
        for x in 0..n {
            let c = map[y][x];
            if c == '#' {
                table[x] = y + 1;
            } else if c == 'O' {
                map[y][x] = '.';
                map[table[x]][x] = 'O';
                table[x] += 1;
            }
        }
    }

    // west
    for y in 0..n {
        let mut table = 0;
        for x in 0..n {
            let c = map[y][x];
            if c == '#' {
                table = x + 1;
            } else if c == 'O' {
                map[y][x] = '.';
                map[y][table] = 'O';
                table += 1;
            }
        }
    }

    // south
    let mut table = vec![n - 1; n];
    for y in 0..n {
        let y = n - 1 - y;
        for x in 0..n {
            let c = map[y][x];
            if c == '#' {
                table[x] = y - 1;
            } else if c == 'O' {
                map[y][x] = '.';
                map[table[x]][x] = 'O';
                table[x] -= 1;
            }
        }
    }

    // east
    let mut id: usize = 0;
    for y in 0..n {
        let mut table = n - 1;
        for x in 0..n {
            let x = n - 1 - x;
            let c = map[y][x];
            if c == '#' {
                table = x - 1;
            } else if c == 'O' {
                id = id.wrapping_add(x + y);
                map[y][x] = '.';
                map[y][table] = 'O';
                table -= 1;
            }
        }
        id = id.swap_bytes();
    }
    id as i64
}

// only works for square matrices
fn transform<T: Copy>(vec: &mut Vec<Vec<T>>) {
    for i in 0..vec.len() {
        for j in i..vec.len() {
            let tmp = vec[i][j];
            vec[i][j] = vec[j][i];
            vec[j][i] = tmp;
        }
    }
}
