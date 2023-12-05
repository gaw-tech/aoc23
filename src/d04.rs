use std::fs;

pub fn solve() {
    let file_path = "input/day04.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents: Vec<&str> = contents.split("\n").collect();

    let mut res1 = 0;
    for card in contents {
        let mut card = card.split(":").last().unwrap().split("|");
        let winning_numbers: Vec<&str> = card.next().unwrap().split(" ").filter(|e| e.len() > 0).collect();
        let drawn_numbers = card.next().unwrap().split(" ").filter(|e| e.len() > 0);
        
        let mut matches = 0;
        for number in drawn_numbers {
            if number.len() > 0 && winning_numbers.contains(&number){
                matches += 1;
            }
        }   
        if matches > 0 {
            res1 += 2i32.pow(matches - 1);
        }
    }

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents: Vec<&str> = contents.split("\n").collect();
    let mut card_id = 0;
    let mut card_amounts: Vec<i32> = vec![1;contents.len()];
    for card in contents {
        let mut card = card.split(":").last().unwrap().split("|");
        let winning_numbers: Vec<&str> = card.next().unwrap().split(" ").filter(|e| e.len() > 0).collect();
        let drawn_numbers = card.next().unwrap().split(" ").filter(|e| e.len() > 0);

        let mut matches = 0;
        for number in drawn_numbers {
            if number.len() > 0 && winning_numbers.contains(&number){
                matches += 1;
            }
        }

        while matches > 0 {
            card_amounts[(card_id + matches) as usize] += card_amounts[card_id as usize];
            matches -= 1; 
        }

        card_id += 1;
    }
    let res2 = card_amounts.iter().fold(0, |a, e| a + e);

    println!("Day 04:\nTask 1:{res1:8}\nTask 2:{res2:8}");
}
