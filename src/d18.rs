use std::fs;

pub fn solve() {
    let file_path = "input/day18.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents = contents.split("\n").filter(|e| e.len() > 0);

    let trench: Vec<(i32, i32, i32)> = contents.fold(vec![(0, 0, 0)], |mut a, e| {
        let mut e = e.split(" ");
        let direction = e.next().unwrap().chars().next().unwrap();
        let distance = e.next().unwrap().parse::<i32>().unwrap();
        let color = i32::from_str_radix(
            &e.next()
                .unwrap()
                .replace("#", "")
                .replace("(", "")
                .replace(")", ""),
            16,
        )
        .unwrap();
        let (mut x, mut y, _) = *a.last().unwrap();
        for _ in 0..distance {
            match direction {
                'U' => {
                    y -= 1;
                }
                'R' => {
                    x += 1;
                }
                'D' => {
                    y += 1;
                }
                'L' => {
                    x -= 1;
                }
                _ => break,
            }
            a.push((x, y, color));
        }
        a
    });

    let mut x_min = 0;
    let mut y_min = 0;
    let mut x_max = 0;
    let mut y_max = 0;

    for (x, y, _) in &trench {
        x_min = x_min.min(*x);
        y_min = y_min.min(*y);
        x_max = x_max.max(*x);
        y_max = y_max.max(*y);
    }

    let mut trench = trench
        .iter()
        .map(|(x, y, c)| (*x - x_min, *y - y_min, *c))
        .fold(
            vec![vec![0; (x_max - x_min + 1) as usize]; (y_max - y_min + 1) as usize],
            |mut a, (x, y, c)| {
                a[y as usize][x as usize] = c;
                a
            },
        );

    let mut start = 0;
    for i in 0..trench[0].len() {
        if trench[0][i] != 0 {
            start = i + 1;
            break;
        }
    }

    let mut queue = vec![(start, 1)];
    while !queue.is_empty() {
        let (x, y) = queue.pop().unwrap();
        if trench[y][x] == 0 {
            trench[y][x] = 1;
            queue.push((x - 1, y));
            queue.push((x + 1, y));
            queue.push((x, y + 1));
            queue.push((x, y - 1));
        }
    }

    let mut res1 = 0;
    for l in &trench {
        for e in l {
            if *e != 0 {
                res1 += 1;
            }
        }
    }

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let instructions = contents.split("\n").filter(|e| e.len() > 0).map(|e| {
        let mut e = e.split(" ");
        let _ = e.next();
        let _ = e.next();
        let e = e
            .next()
            .unwrap()
            .replace("#", "")
            .replace("(", "")
            .replace(")", "");
        let n = e.len();
        (
            i64::from_str_radix(&e[0..n - 1], 16).unwrap(),
            e[n - 1..].parse::<i64>().unwrap(),
        )
    });

    let trench = instructions.fold(vec![(0, 0)], |mut a, (distance, direction)| {
        let (mut x, mut y) = a.last().unwrap();
        match direction {
            0 => x += distance,
            1 => y += distance,
            2 => x -= distance,
            3 => y -= distance,
            _ => (),
        }
        a.push((x, y));
        a
    });

    let mut res2 = 0;
    let n = trench.len();
    let mut sum = 0;
    for i in 0..n {
        let j = (i + 1) % n;
        let (x1, y1) = trench[i];
        let (x2, y2) = trench[j];
        res2 += x1 * y2 - x2 * y1;
        sum += (x2 - x1 + y2 - y1).abs();
    }
    sum = sum / 2;
    res2 = res2 / 2 + sum + 1;

    println!("Day 18:\nTask 1:{res1:16}\nTask 2:{res2:16}");
}
