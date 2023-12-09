use std::fs;

pub fn solve() {
    let file_path = "input/day09.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents = contents.split("\n").filter(|e| e.len() > 0).map(|e| e.split(" ").map(|e| e.parse::<i64>().unwrap()).collect::<Vec<i64>>());
    
    let res1: i64 = contents.clone().map(|e| extrapolate1(e)).sum();
    let res2: i64 = contents.map(|e| extrapolate2(e)).sum();

    println!("Day 09:\nTask 1:{res1:16}\nTask 2:{res2:16}");
}

fn extrapolate1(vec: Vec<i64>) -> i64 {
    let mut vec = vec;
    let mut lists = vec![vec.clone()];
    while vec.iter().any(|e| *e != 0) {
        let mut iter = vec.iter();
        let first = iter.next().unwrap();
        vec = iter.fold((first, Vec::new()), |(last, mut a), e| {a.push(e - last);(e, a)}).1;
        lists.push(vec.clone());
    }
    lists.iter().map(|e| e.last().unwrap()).sum()
}

fn extrapolate2(vec: Vec<i64>) -> i64 {
    let mut vec = vec;
    let mut lists = vec![vec.clone()];
    while vec.iter().any(|e| *e != 0) {
        let mut iter = vec.iter();
        let first = iter.next().unwrap();
        vec = iter.fold((first, Vec::new()), |(last, mut a), e| {a.push(e - last);(e, a)}).1;
        lists.push(vec.clone());
    }
    let lists = lists.iter().map(|e| e.first().unwrap());
    lists.rev().fold(0, |a, e| e - a)
}

//password: v92!y-D1Cds2Z-1
