use std::fs;

pub fn solve() {
    let file_path = "input/day11.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents: Vec<&str> = contents.split("\n").filter(|e| e.len() > 0).collect();

    let mut empty_columns = vec![true;contents[0].len()];
    for i in 0..empty_columns.len() {
        empty_columns[i] = contents.iter().map(|e| e.chars().collect::<Vec<char>>()[i]).all(|e| e == '.');
    }

    let mut empty_rows = vec![true;contents.len()];
    for (i, r) in contents.iter().enumerate() {
        empty_rows[i] = !r.contains('#');
    }
    
    let galaxies: Vec<(usize, usize)> = contents.iter().enumerate().map(|(y,e)| {
        e.chars().enumerate().filter(|(_, g)| *g == '#').map(|(x, _)| (y, x)).collect::<Vec<(usize, usize)>>()
    }).flatten().collect();

    let res1 = count(&galaxies, &empty_columns, &empty_rows, 2);
    let res2 = count(&galaxies, &empty_columns, &empty_rows, 1_000_000);

    println!("Day 11:\nTask 1:{res1:16}\nTask 2:{res2:16}");
}

fn count(galaxies: &Vec<(usize, usize)>, empty_columns: &Vec<bool>, empty_rows: &Vec<bool>, empty_cost: i64) -> i64 {
    let mut res = 0;
    for (a, (a1, a2)) in galaxies.iter().enumerate() {
        for (b, (b1, b2)) in galaxies.iter().enumerate() {
            if a > b {
                let yma = *a1.max(b1) as i64;
                let ymi = *a1.min(b1) as i64;
                let xma = *a2.max(b2) as i64;
                let xmi = *a2.min(b2) as i64;
                let mut empty = 0;
                for i in xmi..xma {
                    if empty_columns[i as usize] {
                        empty += 1;
                    }
                }
                for i in ymi..yma {
                    if empty_rows[i as usize] {
                        empty += 1;
                    }
                }
                res += xma - xmi + yma - ymi + empty_cost * empty - empty
            }
        }
    }
    res
}