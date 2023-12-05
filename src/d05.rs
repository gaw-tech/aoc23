use std::fs;

pub fn solve() {
    let file_path = "input/day05.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents: Vec<&str> = contents.split("\n\n").collect();
    
    let mut seeds: Vec<i64> = contents.first().unwrap().split(" ").skip(1).map(|e| e.parse::<i64>().unwrap()).collect();
    let maps = parse_maps(contents[1..].to_vec());
    let start_seeds = seeds.clone();
    for map in &maps {
        seeds = seeds.iter().map(|e| {
            for (o, i, r) in map {
                if e >= &i && e < &(i + r) {
                    return e - i + o;
                }
            }
            *e
        }).collect();
    }
    let res1 = seeds.iter().min().unwrap();

    let n = start_seeds.len();
    let mut i = 0;
    let mut seeds = Vec::new();
    while i < n {
        seeds.push((start_seeds[i], start_seeds[i+1]));
        i += 2;
    }
    for map in maps {
        let mut new_seeds = Vec::new();
        for (start, range) in seeds {
            let mut covered = Vec::new();
            for (o, i, r) in map.clone() {
                //start in range
                if start >= i && start < i + r {
                    //end in range
                    if start + range > i && start + range <= i + r {
                        new_seeds.push((start - i + o, range));
                        covered.push((start, range));
                    //end not in range
                    } else {
                        new_seeds.push((start - i + o, r + i - start));
                        covered.push((start, r - start + i));
                    }
                //start not in range
                } else {
                    //end in range
                    if start + range > i && start + range <= i + r {
                        new_seeds.push((o, start + range - i));
                        covered.push((i, start + range - i));
                    //seeds and map do overlap
                    } else if start < i && start + range > i + r{
                        new_seeds.push((o, r));
                        covered.push((i,r));
                    }
                }
            }
            new_seeds.append(&mut get_uncovered(start, range, covered));
        }
        seeds = new_seeds;
        seeds.sort();
        seeds.dedup();
        let mut prev = 0;
        seeds = seeds.iter().fold(Vec::new(), |mut a, (start,range)| {
            if *start == prev {
                a.pop();
                a.push((*start, *range));
            } else {
                a.push((*start, *range));
            }
            prev = *start;
            a
        });
    }
    let res2 = seeds.iter().map(|(e,_)| e).min().unwrap();

    println!("Day 05:\nTask 1:{res1:8}\nTask 2:{res2:8}");
}

fn parse_maps(input: Vec<&str>) -> Vec<Vec<(i64, i64, i64)>> {
    let mut maps = Vec::new();
    for s in input {
        let s = s.split("\n").skip(1);
        let mut map = Vec::new();
        for m in s {
            let m: Vec<i64> = m.split(" ").filter(|e| e.len() > 0).map(|e| e.parse::<i64>().unwrap()).collect();
            map.push((m[0],m[1],m[2]));
        }
        maps.push(map);
    }
    maps
}

fn get_uncovered(mut start: i64, range: i64, mut covered: Vec<(i64,i64)>) -> Vec<(i64,i64)> {
    let mut uncovered = Vec::new();
    covered.push((start + range, 0));
    covered.sort();
    for (i, r) in covered {
        uncovered.push((start, i - start));
        start = i + r;
    }

    uncovered = uncovered.iter().filter(|(_, r)| r > &0).map(|e| *e).collect();
    uncovered
}