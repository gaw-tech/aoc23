use std::fs;

pub fn solve() {
    let file_path = "input/day21.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut map: Vec<Vec<char>> = contents
        .split("\n")
        .filter(|e| e.len() > 0)
        .map(|e| e.chars().collect())
        .collect();

    let n = map.len();
    for _ in 0..64 {
        let mut step = vec![vec!['.'; n]; n];
        for y in 0..n {
            for x in 0..n {
                if map[y][x] == '#' {
                    step[y][x] = '#';
                } else if (y.wrapping_sub(1) < n && map[y - 1][x] == 'S')
                    || (y + 1 < n && map[y + 1][x] == 'S')
                    || (x.wrapping_sub(1) < n && map[y][x - 1] == 'S')
                    || (x + 1 < n && map[y][x + 1] == 'S')
                {
                    step[y][x] = 'S';
                }
            }
        }
        map = step;
    }
    
    let res1: usize = map.iter().map(|e| e.iter().filter(|e| **e == 'S').count()).sum();
    // y are values at 65 + 131 * x and then its possible to extrapolate with lagrange
    let (x1, y1) = (65, 3778);
    let (x2, y2) = (196, 33833);
    let (x3, y3) = (327, 93864);
    let x = 26501365i64;
    let res2 = y1 * (((x - x2) * (x - x3)) / ((x1 - x2) * (x1 - x3)))
    + y2 * (((x - x1) * (x - x3)) / ((x2 - x1) * (x2 - x3)))
    + y3 * (((x - x1) * (x - x2)) / ((x3 - x1) * (x3 - x2)));
    
    println!("Day 21:\nTask 1:{res1:16}\nTask 2:{res2:16}");
}
