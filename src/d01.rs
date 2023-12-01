use std::fs;

pub fn solve() {
    let file_path = "input/day01.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents = contents.split("\n");

    let mut res1 = 0;
    for s in contents {
        let s: Vec<char> = s.chars().filter(|&c| c.is_digit(10)).collect();
        if s.len() < 1 {
            continue;
        };
        let s = String::from(s[0]) + &String::from(s[s.len() - 1]);
        res1 += s.parse::<i32>().unwrap();
    }

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents = contents.split("\n");

    let mut res2 = 0;
    for s in contents {
        res2 += get_numbers(s);
    }

    println!("Day 01:\nTask 1:{res1:6}\nTask 2:{res2:6}");
}

fn get_numbers(string: &str) -> i32 {
    if string.len() < 1 {
        return 0;
    }
    let mut string = string.clone();
    let mut res = 0;
    loop {
        let d = string.chars().next().unwrap();
        if d.is_digit(10) {
            res += String::from(d).parse::<i32>().unwrap() * 10;
            break;
        }
        if string.starts_with("one") {
            res += 10;
            break;
        }
        if string.starts_with("two") {
            res += 20;
            break;
        }
        if string.starts_with("three") {
            res += 30;
            break;
        }
        if string.starts_with("four") {
            res += 40;
            break;
        }
        if string.starts_with("five") {
            res += 50;
            break;
        }
        if string.starts_with("six") {
            res += 60;
            break;
        }
        if string.starts_with("seven") {
            res += 70;
            break;
        }
        if string.starts_with("eight") {
            res += 80;
            break;
        }
        if string.starts_with("nine") {
            res += 90;
            break;
        }
        string = &string[1..];
    }
    if string.len() < 1 {
        res += res/10;
        return res;
    }
    loop {
        let d = string.chars().last().unwrap();
        if d.is_digit(10) {
            res += String::from(d).parse::<i32>().unwrap() * 1;
            break;
        }
        if string.ends_with("one") {
            res += 1;
            break;
        }
        if string.ends_with("two") {
            res += 2;
            break;
        }
        if string.ends_with("three") {
            res += 3;
            break;
        }
        if string.ends_with("four") {
            res += 4;
            break;
        }
        if string.ends_with("five") {
            res += 5;
            break;
        }
        if string.ends_with("six") {
            res += 6;
            break;
        }
        if string.ends_with("seven") {
            res += 7;
            break;
        }
        if string.ends_with("eight") {
            res += 8;
            break;
        }
        if string.ends_with("nine") {
            res += 9;
            break;
        }
        string = &string[..string.len()-1];
    }
    res
}
