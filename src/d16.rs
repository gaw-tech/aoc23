use std::fs;

pub fn solve() {
    let file_path = "input/day16.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mirrors: Vec<Vec<char>> = contents
        .split("\n")
        .filter(|e| e.len() > 0)
        .map(|e| e.chars().collect())
        .collect();
    let n = mirrors.len();

    let mut beams = vec![vec![(false, false, false, false); n]; n];
    beam('r', 0, 0, &mut beams, &mirrors);

    let res1 = count(&mut beams);

    let mut max = 0;
    for y in 0..n {
        for x in 0..n {
            // r
            let mut beams = vec![vec![(false, false, false, false); n]; n];
            beam('r', 0, y, &mut beams, &mirrors);
            max = max.max(count(&mut beams));
            // d
            let mut beams = vec![vec![(false, false, false, false); n]; n];
            beam('d', x, 0, &mut beams, &mirrors);
            max = max.max(count(&mut beams));
            // l
            let mut beams = vec![vec![(false, false, false, false); n]; n];
            beam('l', n - 1, y, &mut beams, &mirrors);
            max = max.max(count(&mut beams));
            // u
            let mut beams = vec![vec![(false, false, false, false); n]; n];
            beam('u', x, n - 1, &mut beams, &mirrors);
            max = max.max(count(&mut beams));
        }
    }
    let res2 = max;

    println!("Day 16:\nTask 1:{res1:16}\nTask 2:{res2:16}");
}

fn beam(
    direction: char,
    x: usize,
    y: usize,
    beams: &mut Vec<Vec<(bool, bool, bool, bool)>>,
    mirrors: &Vec<Vec<char>>,
) {
    let n = beams.len();
    if x < n && y < n {
        match direction {
            'u' => {
                if !beams[y][x].0 {
                    beams[y][x].0 = true;
                    let cur = mirrors[y][x];
                    match cur {
                        '.' => {
                            beam('u', x, y.wrapping_sub(1), beams, mirrors);
                        }
                        '/' => {
                            beam('r', x + 1, y, beams, mirrors);
                        }
                        '\\' => {
                            beam('l', x.wrapping_sub(1), y, beams, mirrors);
                        }
                        '|' => {
                            beam('u', x, y.wrapping_sub(1), beams, mirrors);
                        }
                        '-' => {
                            beam('l', x.wrapping_sub(1), y, beams, mirrors);
                            beam('r', x + 1, y, beams, mirrors);
                        }
                        _ => (),
                    }
                }
            }
            'r' => {
                if !beams[y][x].1 {
                    beams[y][x].1 = true;
                    let cur = mirrors[y][x];
                    match cur {
                        '.' => {
                            beam('r', x + 1, y, beams, mirrors);
                        }
                        '/' => {
                            beam('u', x, y.wrapping_sub(1), beams, mirrors);
                        }
                        '\\' => {
                            beam('d', x, y + 1, beams, mirrors);
                        }
                        '|' => {
                            beam('u', x, y.wrapping_sub(1), beams, mirrors);
                            beam('d', x, y + 1, beams, mirrors);
                        }
                        '-' => {
                            beam('r', x + 1, y, beams, mirrors);
                        }
                        _ => (),
                    }
                }
            }
            'd' => {
                if !beams[y][x].2 {
                    beams[y][x].2 = true;
                    let cur = mirrors[y][x];
                    match cur {
                        '.' => {
                            beam('d', x, y + 1, beams, mirrors);
                        }
                        '/' => {
                            beam('l', x.wrapping_sub(1), y, beams, mirrors);
                        }
                        '\\' => {
                            beam('r', x + 1, y, beams, mirrors);
                        }
                        '|' => {
                            beam('d', x, y + 1, beams, mirrors);
                        }
                        '-' => {
                            beam('r', x + 1, y, beams, mirrors);
                            beam('l', x.wrapping_sub(1), y, beams, mirrors);
                        }
                        _ => (),
                    }
                }
            }
            'l' => {
                if !beams[y][x].3 {
                    beams[y][x].3 = true;
                    let cur = mirrors[y][x];
                    match cur {
                        '.' => {
                            beam('l', x.wrapping_sub(1), y, beams, mirrors);
                        }
                        '/' => {
                            beam('d', x, y + 1, beams, mirrors);
                        }
                        '\\' => {
                            beam('u', x, y.wrapping_sub(1), beams, mirrors);
                        }
                        '|' => {
                            beam('u', x, y.wrapping_sub(1), beams, mirrors);
                            beam('d', x, y + 1, beams, mirrors);
                        }
                        '-' => {
                            beam('l', x.wrapping_sub(1), y, beams, mirrors);
                        }
                        _ => (),
                    }
                }
            }
            _ => (),
        }
    }
}

fn count(beams: &mut Vec<Vec<(bool, bool, bool, bool)>>) -> i32 {
    beams.iter().fold(0, |a, e| {
        a + e
            .iter()
            .fold(0, |a, e| if e.0 || e.1 || e.2 || e.3 { a + 1 } else { a })
    }) as i32
}
