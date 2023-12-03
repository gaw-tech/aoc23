use std::fs;

pub fn solve() {
    let file_path = "input/day02.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents = contents.split("\n");

    let mut res1 = 0;

    for game in contents {
        let (gid, game) = get_game_id(game);
        let game = game.split(";");
        let mut valid_game = true;
        for set in game {
            if !valid_set(set) {
                valid_game = false;
            }
        }
        if valid_game {
            res1 += gid;
        }
    }

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents = contents.split("\n");

    let mut res2 = 0;

    for game in contents {
        let (gid, game) = get_game_id(game);
        let (r, g, b) = game_min_set(game);
        res2 += r * g * b;
    }

    println!("Day 02:\nTask 1:{res1:6}\nTask 2:{res2:6}");
}

fn get_game_id(game: &str) -> (i32, &str) {
    let mut game = game.split(":");
    let gid = game.next().unwrap();
    (gid.replace("Game ", "").parse::<i32>().unwrap(), game.next().unwrap())
} 

fn valid_set(set: &str) -> bool {
    let set = set.split(",");
    for mut e in set {
        while e.starts_with(" ") {
            e = &e[1..];
        }
        let n = e.split(" ").next().unwrap().parse::<i32>().unwrap();
        if e.contains("red") && n > 12 {
            return false;
        }
        if e.contains("green") && n > 13 {
            return false;
        }
        if e.contains("blue") && n > 14 {
            return false;
        }
    }
    true
}

fn game_min_set(game: &str) -> (i32, i32, i32) {
    let game = game.split(";");
    let mut mr = 0;
    let mut mg = 0;
    let mut mb = 0;
    for set in game {
        let set = set.split(",");
        for mut e in set {
            while e.starts_with(" ") {
                e = &e[1..];
            }
            let n = e.split(" ").next().unwrap().parse::<i32>().unwrap();
            if e.contains("red") {
                mr = mr.max(n);
            }
            if e.contains("green") {
                mg = mg.max(n);
            }
            if e.contains("blue") {
                mb = mb.max(n);
            }
        }
    }
    (mr, mg, mb)
}