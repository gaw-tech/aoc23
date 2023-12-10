use std::fs;

pub fn solve() {
    let file_path = "input/day10.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut contents: Vec<Vec<char>> = contents.split("\n").filter(|e| e.len() > 0).map(|e| e.chars().collect::<Vec<char>>()).collect();

    let mut y = 0;
    let mut x = 0;

    for (sy, line) in contents.iter().enumerate() {
        for (sx, c) in line.iter().enumerate() {
            if *c == 'S' {
                y = sy;
                x = sx;
            }
        } 
    }

    let mut res1 = 0;
    while (x, y) != (0,0) {
        (x, y) = traverse(x, y, &mut contents);
        res1 += 1;
    }
    let res1 = res1 / 2;

    let mut used = contents;
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents: Vec<Vec<char>> = contents.split("\n").filter(|e| e.len() > 0).map(|e| e.chars().collect::<Vec<char>>()).collect();

    used = used.iter().map(|e| e.iter().map(|e| if *e != 'W' {' '} else {*e}).collect()).collect();

    for (y, l) in contents.clone().iter().enumerate() {
        for (x, c) in l.clone().iter().enumerate() {
            if used[y][x] == 'W' {
                used[y][x] = *c;
                if *c == 'S' {
                    used[y][x] = '|';
                }
            }
        }
    }

    let mut inside = false;
    let mut res2 = 0;
    for l in used {
        let mut l = l.iter().filter(|e| **e != '-').peekable();
        while let Some(c) = l.next() {
            println!("{c} {inside}");
            if inside && *c == ' ' {
                res2 += 1;
            }
            if *c == '|' {
                inside = !inside;
            }
            if *c == 'F' {
                if **l.peek().unwrap() == 'J' {
                    inside = !inside;
                }
            }
            if *c == 'L' {
                if **l.peek().unwrap() == '7' {
                    inside = !inside;
                }
            }

        }
    }
    
    println!("Day 10:\nTask 1:{res1:16}\nTask 2:{res2:16}");
}

fn traverse(x: usize, y: usize, map: &mut Vec<Vec<char>>) -> (usize, usize) {
    let cur = map[y][x];
    map[y][x] = 'W';
    let top = vec!['7', '|', 'F'];
    let bottom = vec!['J', '|', 'L'];
    let left = vec!['L', '-', 'F'];
    let right = vec!['J', '-', '7'];
    if bottom.contains(&cur) || cur == 'S' {
        if let Some(n) = map.get(y.wrapping_sub(1)) {
            let n = n[x];
            if top.contains(&n) {
                return (x, y.wrapping_sub(1));
            }
        }
    }
    if left.contains(&cur) || cur == 'S' {
        if let Some(n) = map[y].get(x + 1) {
            if right.contains(n) {
                return (x + 1, y);
            }
        }
    }
    if top.contains(&cur) || cur == 'S' {
        if let Some(n) = map.get(y + 1) {
            let n = n[x];
            if bottom.contains(&n) {
                return (x, y + 1);
            }
        }
    }
    if right.contains(&cur) || cur == 'S' {
        if let Some(n) = map[y].get(x.wrapping_sub(1)) {
            if left.contains(n) {
                return (x.wrapping_sub(1), y);
                
            }
        }
    }
    (0,0)
}
