use std::{
    collections::HashMap,
    fs, time::Instant,
};

pub fn solve() {
    let file_path = "input/day22.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut bricks: Vec<((i64, i64, i64), (i64, i64, i64))> = contents
        .split("\n")
        .filter(|e| e.len() > 0)
        .map(|e| {
            let mut e = e.split("~");
            let a: Vec<i64> = e
                .next()
                .unwrap()
                .split(',')
                .map(|e| e.parse().unwrap())
                .collect();
            let b: Vec<i64> = e
                .next()
                .unwrap()
                .split(',')
                .map(|e| e.parse().unwrap())
                .collect();
            ((a[0], a[1], a[2]), (b[0], b[1], b[2]))
        })
        .collect();

    // sort by z
    bricks.sort_by(|(a1, a2), (b1, b2)| {
        ((a1.2, a1.0, a1.1), (a2.2, a2.0, a2.1)).cmp(&((b1.2, b1.0, b1.1), (b2.2, b2.0, b2.1)))
    });

    let mut stack: Vec<((i64, i64, i64), (i64, i64, i64))> = Vec::new();
    for (a1, b1) in bricks {
        // sort by z (second hight is more important)
        stack.sort_by(|(a1, b1), (a2, b2)| {
            ((b1.2, a1.0, b1.1), (a1.2, b1.0, a1.1)).cmp(&((b2.2, a2.0, b2.1), (a2.2, b2.0, a2.1)))
        });
        let mut on_floor = true;
        for i in 0..stack.len() {
            let i = stack.len() - 1 - i;
            let (a2, b2) = stack[i];
            if overlap(&(a1, b1), &(a2, b2)) {
                let dif = a1.2 - b2.2 - 1;
                stack.push(((a1.0, a1.1, a1.2 - dif), (b1.0, b1.1, b1.2 - dif)));
                on_floor = false;
                break;
            }
        }
        if on_floor {
            stack.push(((a1.0, a1.1, 1), (b1.0, b1.1, b1.2 - a1.2 + 1)));
        }
    }

    let mut up: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut down: HashMap<usize, Vec<usize>> = HashMap::new();

    for (i, (a1, b1)) in stack.iter().enumerate() {
        let _ = stack
            .iter()
            .enumerate()
            .filter(|(_, (a2, b2))| {
                if !(a1, b1).eq(&(a2, b2)) && overlap(&(*a1, *b1), &(*a2, *b2)) {
                    b1.2 + 1 == a2.2
                } else {
                    false
                }
            })
            .inspect(|(j, _)| {
                if let Some(l) = up.get_mut(&i) {
                    l.push(*j);
                } else {
                    up.insert(i, vec![*j]);
                }
                if let Some(l) = down.get_mut(&j) {
                    l.push(i);
                } else {
                    down.insert(*j, vec![i]);
                }
            })
            .count();
    }

    let mut res1 = 0;
    for i in 0..stack.len() {
        if let Some(a) = up.get(&i) {
            if !a.iter().map(|e| down.get(e).unwrap().len()).any(|e| e < 2) {
                res1 += 1;
            }
        } else {
            res1 += 1;
        }
    }

    let mut res2 = 0;
    for i in 0..stack.len() {
        let mut removed = HashMap::new();
        if let Some(a) = up.get(&i) {
            let mut queue = a.clone();
            for e in &queue {
                removed.insert(*e, 1);
            }
            while !queue.is_empty() {
                let next = queue.pop().unwrap();
                let next_support = down.get(&next).unwrap().len();
                if *removed.get(&next).unwrap() == next_support {
                    if let Some(supported_by_next) = up.get(&next) {
                        queue.append(&mut supported_by_next.clone());
                        for e in supported_by_next {
                            if let Some(v) = removed.get(e) {
                                removed.insert(*e, v + 1);
                            } else {
                                removed.insert(*e, 1);
                            }
                        }
                    }
                    res2 += 1;
                }
            }
        }
    }

    println!("Day 22:\nTask 1:{res1:16}\nTask 2:{res2:16}");
}

fn overlap(
    (a1, b1): &((i64, i64, i64), (i64, i64, i64)),
    (a2, b2): &((i64, i64, i64), (i64, i64, i64)),
) -> bool {
    let xoverlap = (a1.0 <= a2.0 && b1.0 >= a2.0)
        || (a1.0 <= b2.0 && b1.0 >= b2.0)
        || (a1.0 >= a2.0 && b1.0 <= b2.0);
    let yoverlap = (a1.1 <= a2.1 && b1.1 >= a2.1)
        || (a1.1 <= b2.1 && b1.1 >= b2.1)
        || (a1.1 >= a2.1 && b1.1 <= b2.1);
    xoverlap && yoverlap
}
