use std::fs;

pub fn solve() {
    let file_path = "input/day06.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut contents = contents.split("\n");
    let time = contents.next().unwrap().split(":").last().unwrap().split(" ").filter(|e| e.len() > 0).map(|e| e.parse::<f64>().unwrap());
    let distance = contents.next().unwrap().split(":").last().unwrap().split(" ").filter(|e| e.len() > 0).map(|e| e.parse::<f64>().unwrap());
    let races: Vec<(f64, f64)> = time.zip(distance).collect();
    
    let mut res1 = 1.0;
    for (time, distance) in races {
        let root1 = 0.5 * (time - (time*time - 4.0 * distance).sqrt());
        let root2 = 0.5 * (time + (time*time - 4.0 * distance).sqrt());
        println!("{root1} {root2} {}",root2.ceil() - root1.floor() - 1.0);
        res1 *= root2.ceil() - root1.floor() - 1.;
    }

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut contents = contents.split("\n");
    let time = contents.next().unwrap().split(":").last().unwrap().replace(" ", "").parse::<f64>().unwrap();
    let distance = contents.next().unwrap().split(":").last().unwrap().replace(" ", "").parse::<f64>().unwrap();
    
    let root1 = 0.5 * (time - (time*time - 4.0 * distance).sqrt());
    let root2 = 0.5 * (time + (time*time - 4.0 * distance).sqrt());
    println!("{root1} {root2} {}",root2.ceil() - root1.floor() - 1.0);
    let res2 = root2.ceil() - root1.floor() - 1.0;


    println!("Day 06:\nTask 1:{res1:8}\nTask 2:{res2:8}");
}