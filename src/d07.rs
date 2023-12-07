use std::{fs, collections::HashMap};

pub fn solve() {
    let file_path = "input/day07.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents = contents.chars().map(|x| match x {
        'T' => 'a',
        'J' => 'b',
        'Q' => 'c',
        'K' => 'd',
        'A' => 'e',
        _ => x
    }).collect::<String>();
    let mut contents: Vec<(String, i64)> = contents.split("\n").filter(|e| e.len() > 0).map(|e| {
        let mut e = e.split(" ");
        let hand = e.next().unwrap();
        (add_type(hand.to_string()), e.next().unwrap().parse().unwrap())
    }).collect();
    contents.sort();

    let mut res1 = 0;
    let mut i = 1;
    for (_, bid) in contents {
        res1 += bid * i;
        i += 1;
    }
    
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents = contents.chars().map(|x| match x {
        'T' => 'a',
        'J' => '1',
        'Q' => 'c',
        'K' => 'd',
        'A' => 'e',
        _ => x
    }).collect::<String>();
    let mut contents: Vec<(String, i64)> = contents.split("\n").filter(|e| e.len() > 0).map(|e| {
        let mut e = e.split(" ");
        let hand = e.next().unwrap();
        (add_jtype(hand.to_string()), e.next().unwrap().parse().unwrap())
    }).collect();
    contents.sort();

    let mut res2 = 0;
    let mut i = 1;
    for (_, bid) in contents {
        res2 += bid * i;
        i += 1;
    }

    println!("Day 07:\nTask 1:{res1:12}\nTask 2:{res2:12}");
}

fn add_type(mut hand: String) -> String {
    let t = hand.chars().fold(HashMap::new(), |mut a, e| {
        if a.contains_key(&e) {
            a.insert(e, a.get(&e).unwrap() + 1);
        } else {
            a.insert(e, 1);
        }
        a
    });
    if t.len() == 5 {
        hand = "0".to_string() + &hand;
    } else if t.len() == 4 {
        hand = "1".to_string() + &hand;
    } else if t.len() == 3 {
        //two pairs
        if t.values().collect::<Vec<&i32>>().contains(&&2) {
            hand = "2".to_string() + &hand;
        // three of a kind
        } else {
            hand = "3".to_string() + &hand;
        }
    } else if t.len() == 2 {
        // fullhouse
        if t.values().collect::<Vec<&i32>>().contains(&&3) {
            hand = "5".to_string() + &hand;
        // four of a kind
        } else {
            hand = "6".to_string() + &hand;
        }
    } else {
        hand = "7".to_string() + &hand;
    }
    hand
}

fn add_jtype(mut hand: String) -> String {
    let mut t = hand.chars().fold(HashMap::new(), |mut a, e| {
        if a.contains_key(&e) {
            a.insert(e, a.get(&e).unwrap() + 1);
        } else {
            a.insert(e, 1);
        }
        a
    });
    if t.contains_key(&'1') {
        let n = t.remove(&'1').unwrap();
        let (c, v) = t.clone().drain().fold((' ', 0), |(ca, va), (c, v)| if v > va {(c, v)} else {(ca, va)});
        t.insert(c, v + n);
    }

    if t.len() == 5 {
        hand = "0".to_string() + &hand;
    } else if t.len() == 4 {
        hand = "1".to_string() + &hand;
    } else if t.len() == 3 {
        //two pairs
        if t.values().collect::<Vec<&i32>>().contains(&&2) {
            hand = "2".to_string() + &hand;
        // three of a kind
        } else {
            hand = "3".to_string() + &hand;
        }
    } else if t.len() == 2 {
        // fullhouse
        if t.values().collect::<Vec<&i32>>().contains(&&3) {
            hand = "5".to_string() + &hand;
        // four of a kind
        } else {
            hand = "6".to_string() + &hand;
        }
    } else {
        hand = "7".to_string() + &hand;
    }
    hand
}