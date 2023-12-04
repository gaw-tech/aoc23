use std::fs;

pub fn solve() {
    let file_path = "input/day03.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents: Vec<&str> = contents.split("\n").collect();

    let mut res1 = 0;
    let mut row_index: usize = 0;
    for line in &contents {
        let mut col_index: usize = 0;
        for char in line.chars() {
            if !char.is_digit(10) && char != '.'{
                let mut pl: Vec<char> = contents.get(row_index.wrapping_sub(1)).unwrap_or(&"").chars().collect();
                if pl.get(col_index.wrapping_sub(1)).unwrap_or(&' ').is_digit(10) {
                    res1 += get_nr( col_index.wrapping_sub(1), &mut pl);
                }
                if pl.get(col_index).unwrap_or(&' ').is_digit(10) {
                    res1 += get_nr( col_index, &mut pl);
                }
                if pl.get(col_index.wrapping_add(1)).unwrap_or(&' ').is_digit(10) {
                    res1 += get_nr( col_index.wrapping_add(1), &mut pl);
                }
                let mut cl: Vec<char> = contents.get(row_index).unwrap_or(&"").chars().collect();
                if cl.get(col_index.wrapping_sub(1)).unwrap_or(&' ').is_digit(10) {
                    res1 += get_nr(col_index.wrapping_sub(1), &mut cl);
                }
                if cl.get(col_index.wrapping_add(1)).unwrap_or(&' ').is_digit(10) {
                    res1 += get_nr(col_index.wrapping_add(1), &mut cl);
                }
                let mut nl: Vec<char> = contents.get(row_index.wrapping_add(1)).unwrap_or(&"").chars().collect();
                if nl.get(col_index.wrapping_sub(1)).unwrap_or(&' ').is_digit(10) {
                    res1 += get_nr( col_index.wrapping_sub(1), &mut nl);
                }
                if nl.get(col_index).unwrap_or(&' ').is_digit(10) {
                    res1 += get_nr( col_index, &mut nl);
                }
                if nl.get(col_index.wrapping_add(1)).unwrap_or(&' ').is_digit(10) {
                    res1 += get_nr( col_index.wrapping_add(1), &mut nl);
                }
            }
            col_index += 1;
        }
        row_index += 1;
    }

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents: Vec<&str> = contents.split("\n").collect();

    let mut res2 = 0;
    let mut row_index: usize = 0;
    for line in &contents {
        let mut col_index: usize = 0;
        for char in line.chars() {
            if !char.is_digit(10) && char != '.'{
                let mut number_of_numbers = 0;
                let mut gear_ratio = 1;
                let mut pl: Vec<char> = contents.get(row_index.wrapping_sub(1)).unwrap_or(&"").chars().collect();
                if pl.get(col_index.wrapping_sub(1)).unwrap_or(&' ').is_digit(10) {
                    gear_ratio *= get_nr( col_index.wrapping_sub(1), &mut pl);
                    number_of_numbers += 1;
                }
                if pl.get(col_index).unwrap_or(&' ').is_digit(10) {
                    gear_ratio *= get_nr( col_index, &mut pl);
                    number_of_numbers += 1;
                }
                if pl.get(col_index.wrapping_add(1)).unwrap_or(&' ').is_digit(10) {
                    gear_ratio *= get_nr( col_index.wrapping_add(1), &mut pl);
                    number_of_numbers += 1;
                }
                let mut cl: Vec<char> = contents.get(row_index).unwrap_or(&"").chars().collect();
                if cl.get(col_index.wrapping_sub(1)).unwrap_or(&' ').is_digit(10) {
                    gear_ratio *= get_nr(col_index.wrapping_sub(1), &mut cl);
                    number_of_numbers += 1;
                }
                if cl.get(col_index.wrapping_add(1)).unwrap_or(&' ').is_digit(10) {
                    gear_ratio *= get_nr(col_index.wrapping_add(1), &mut cl);
                    number_of_numbers += 1;
                }
                let mut nl: Vec<char> = contents.get(row_index.wrapping_add(1)).unwrap_or(&"").chars().collect();
                if nl.get(col_index.wrapping_sub(1)).unwrap_or(&' ').is_digit(10) {
                    gear_ratio *= get_nr( col_index.wrapping_sub(1), &mut nl);
                    number_of_numbers += 1;
                }
                if nl.get(col_index).unwrap_or(&' ').is_digit(10) {
                    gear_ratio *= get_nr( col_index, &mut nl);
                    number_of_numbers += 1;
                }
                if nl.get(col_index.wrapping_add(1)).unwrap_or(&' ').is_digit(10) {
                    gear_ratio *= get_nr( col_index.wrapping_add(1), &mut nl);
                    number_of_numbers += 1;
                }
                if number_of_numbers == 2 {
                    res2 += gear_ratio;
                }
            }
            col_index += 1;
        }
        row_index += 1;
    }

    println!("Day 03:\nTask 1:{res1:8}\nTask 2:{res2:8}");
}

fn get_nr(mut col: usize, c: &mut Vec<char>) -> i32 {
    while c.get(col.wrapping_sub(1)).unwrap_or(&' ').is_digit(10) {
        col -= 1;
    }
    let mut res = 0;
    while c.get(col).unwrap_or(&' ').is_digit(10) {
        let digit = c.get(col).unwrap();
        res = 10 * res + String::from(*digit).parse::<i32>().unwrap();
        c[col] = '.';
        col += 1;
    }
    res
}