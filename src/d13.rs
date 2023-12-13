use std::fs;

pub fn solve() {
    let file_path = "input/day13.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents = contents
        .split("\n\n")
        .filter(|e| e.len() > 0)
        .map(|e| {
            let e: Vec<&str> = e.split("\n").filter(|e| e.len() > 0).collect();
            let mut i = 0;
            let mut vertical = false;
            while i < e[0].len() - 1 && !vertical {
                i += 1;
                vertical = e.iter().all(|e| {
                    let left = i - i.min(e.len() - i);
                    let right = i + i.min(e.len() - i);
                    let left = e[left..i].chars().rev().collect::<Vec<char>>();
                    let right = e[i..right].chars().collect::<Vec<char>>();
                    left.eq(&right)
                });
            }
            let mut j = 0;
            let mut horizontal = false;
            while j < e.len() - 1 && !horizontal {
                j += 1;
                let mut tmp = true;
                for h in 0..(j.min(e.len() - j)) {
                    tmp = tmp && e[j - h - 1].eq(e[j + h])
                }
                horizontal = tmp;
            }
            if vertical {
                ('v', i)
            } else {
                ('h', j)
            }
        })
        .fold(0, |a, (d, v)| if d == 'h' { a + 100 * v } else { a + v });

    let res1 = contents;

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents = contents
        .split("\n\n")
        .filter(|e| e.len() > 0)
        .map(|e| {
            let e: Vec<&str> = e.split("\n").filter(|e| e.len() > 0).collect();let mut i = 0;

            let mut vertical = false;
            while i < e[0].len() - 1 && !vertical {
                i += 1;
                vertical = e.iter().all(|e| {
                    let left = i - i.min(e.len() - i);
                    let right = i + i.min(e.len() - i);
                    let left = e[left..i].chars().rev().collect::<Vec<char>>();
                    let right = e[i..right].chars().collect::<Vec<char>>();
                    left.eq(&right)
                });
            }
            let mut j = 0;
            let mut horizontal = false;
            while j < e.len() - 1 && !horizontal {
                j += 1;
                let mut tmp = true;
                for h in 0..(j.min(e.len() - j)) {
                    tmp = tmp && e[j - h - 1].eq(e[j + h])
                }
                horizontal = tmp;
            }
            let (d, v);
            if vertical {
                (d, v) = ('v', i)
            } else {
                (d, v) = ('h', j)
            }

            let e = e.iter().map(|e| e.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
            let mut vertical = false;
            let mut horizontal = false;
            let mut i = 0;
            let mut j = 0;
            let mut x = 0;
            while !vertical && !horizontal {
                let mut e = e.clone();
                let y = x / e[0].len();
                let z = x % e[0].len();
                x += 1;
                let mut c = e[y][z];
                if c == '.' {
                    c = '#';
                } else {
                    c = '.';
                }
                e[y][z] = c;
                i = 0;
                j = 0;
                while i < e[0].len() - 1 && !vertical {
                    i += 1;
                    vertical = e.iter().all(|e| {
                        let left = i - i.min(e.len() - i);
                        let right = i + i.min(e.len() - i);
                        let left = e[left..i].to_vec();
                        let right = e[i..right].iter().collect::<Vec<&char>>();
                        left.iter().rev().collect::<Vec<&char>>().eq(&right)
                    });
                    if vertical && d == 'v' && v == i {
                        vertical = false;
                    }
                }
                while j < e.len() - 1 && !horizontal {
                    j += 1;
                    let mut tmp = true;
                    for h in 0..(j.min(e.len() - j)) {
                        tmp = tmp && e[j - h - 1].eq(&e[j + h])
                    }
                    horizontal = tmp;
                    if horizontal && d == 'h' && v == j {
                        horizontal = false;
                    }
                }
            }
            if vertical {
                ('v', i)
            } else {
                ('h', j)
            }
        })
        .inspect(|e| println!("{e:?}"))
        .fold(0, |a, (d, v)| if d == 'h' { a + 100 * v } else { a + v });

    let res2 = contents;

    println!("Day 13:\nTask 1:{res1:16}\nTask 2:{res2:16}");
}
