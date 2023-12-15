use std::fs;

pub fn solve() {
    let file_path = "input/day15.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents = contents.replace("\n", "");
    
    let res1: u32 = contents.clone().split(",").map(|e| hash(e)).sum();

    let mut map = vec![vec![];256];

    for i in contents.split(",") {
        if i.contains("-") {
            let mut i = i.split("-");
            let label = i.next().unwrap();
            let hash = hash(label);
            let b: &mut Vec<(&str, usize)> = &mut map[hash as usize];
            for i in 0..b.len() {
                if label.eq(b[i].0) {
                    b.remove(i);
                    break;
                }
            }
        } else {
            let mut i = i.split("=");
            let label = i.next().unwrap();
            let hash = hash(label);
            let focal_length: usize = i.next().unwrap().parse().unwrap();
            let b: &mut Vec<(&str, usize)> = &mut map[hash as usize];
            let mut replaced = false;
            for i in 0..b.len() {
                if label.eq(b[i].0) {
                    b[i] = (label, focal_length);
                    replaced = true;
                }
            }
            if !replaced {
                b.push((label, focal_length));
            }
        }
    }

    let res2 = map.iter().enumerate().fold(0, |a, (i, b)| {
        a + b.iter().enumerate().fold(0, |a, (j, (_, v))| a + (i + 1) * (j + 1) * v)
    });

    println!("Day 15:\nTask 1:{res1:16}\nTask 2:{res2:16}");
}

fn hash(str: &str) -> u32 {
    str.chars().fold(0, |a, e| {
        ((a + e as u32) * 17) % 256
    })
}
