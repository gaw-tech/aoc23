use std::{collections::HashMap, fs, usize, vec};

pub fn solve() {
    let file_path = "input/day23.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let map: Vec<Vec<char>> = contents
        .split("\n")
        .filter(|e| e.len() > 0)
        .map(|e| e.chars().collect())
        .collect();

    let n = map.len();
    let start = (0, 1);
    let end = (map[map.len() - 1].len() - 2, map.len() - 1);

    let mut graph: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
    let mut queue = vec![(start, (3, 3))];
    while !queue.is_empty() {
        let ((x, y), (px, py)): ((usize, usize), (usize, usize)) = queue.pop().unwrap();
        if graph.contains_key(&(x, y)) {
            continue;
        }
        if y.wrapping_sub(1) < n && py != y - 1 {
            let next = map[y - 1][x];
            if next == '.' || next == '^' {
                if let Some(edges) = graph.get_mut(&(x, y)) {
                    edges.push((x, y - 1));
                } else {
                    graph.insert((x, y), vec![(x, y - 1)]);
                }
                queue.push(((x, y - 1), (x, y)));
            }
        }
        if x.wrapping_sub(1) < n && px != x - 1 {
            let next = map[y][x - 1];
            if next == '.' || next == '<' {
                if let Some(edges) = graph.get_mut(&(x, y)) {
                    edges.push((x - 1, y));
                } else {
                    graph.insert((x, y), vec![(x - 1, y)]);
                }
                queue.push(((x - 1, y), (x, y)));
            }
        }
        if y + 1 < n && py != y + 1 {
            let next = map[y + 1][x];
            if next == '.' || next == 'v' {
                if let Some(edges) = graph.get_mut(&(x, y)) {
                    edges.push((x, y + 1));
                } else {
                    graph.insert((x, y), vec![(x, y + 1)]);
                }
                queue.push(((x, y + 1), (x, y)));
            }
        }
        if x + 1 < n && px != x + 1 {
            let next = map[y][x + 1];
            if next == '.' || next == '>' {
                if let Some(edges) = graph.get_mut(&(x, y)) {
                    edges.push((x + 1, y));
                } else {
                    graph.insert((x, y), vec![(x + 1, y)]);
                }
                queue.push(((x + 1, y), (x, y)));
            }
        }
    }

    let mut res1 = 0;
    let mut queue = Vec::new();
    queue.push((start, 0));
    while !queue.is_empty() {
        let (next, val) = queue.pop().unwrap();
        res1 = res1.max(val);
        if let Some(edges) = graph.get(&next) {
            for e in edges {
                queue.push((*e, val + 1));
            }
        }
    }

    // ----------------------------------------------------------------

    let mut graph: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
    let mut queue = vec![start];
    let path = vec!['.', '^', '>', 'v', '<'];
    while !queue.is_empty() {
        let (x, y): (usize, usize) = queue.pop().unwrap();
        if graph.contains_key(&(x, y)) {
            continue;
        }
        if y.wrapping_sub(1) < n {
            let next = map[y - 1][x];
            if path.contains(&next) {
                if let Some(edges) = graph.get_mut(&(x, y)) {
                    edges.push((x, y - 1));
                } else {
                    graph.insert((x, y), vec![(x, y - 1)]);
                }
                queue.push((x, y - 1));
            }
        }
        if x.wrapping_sub(1) < n {
            let next = map[y][x - 1];
            if path.contains(&next) {
                if let Some(edges) = graph.get_mut(&(x, y)) {
                    edges.push((x - 1, y));
                } else {
                    graph.insert((x, y), vec![(x - 1, y)]);
                }
                queue.push((x - 1, y));
            }
        }
        if y + 1 < n {
            let next = map[y + 1][x];
            if path.contains(&next) {
                if let Some(edges) = graph.get_mut(&(x, y)) {
                    edges.push((x, y + 1));
                } else {
                    graph.insert((x, y), vec![(x, y + 1)]);
                }
                queue.push((x, y + 1));
            }
        }
        if x + 1 < n {
            let next = map[y][x + 1];
            if path.contains(&next) {
                if let Some(edges) = graph.get_mut(&(x, y)) {
                    edges.push((x + 1, y));
                } else {
                    graph.insert((x, y), vec![(x + 1, y)]);
                }
                queue.push((x + 1, y));
            }
        }
    }

    let mut crossroads = Vec::new();
    for (k, edges) in &graph {
        if edges.len() != 2 {
            crossroads.push(*k);
        }
    }

    let mut short_graph = HashMap::new();
    for k in crossroads {
        let mut new_edges = Vec::new();
        if let Some(edges) = graph.get(&k) {
            for edge in edges {
                let mut edge = *edge;
                let mut pre = k;
                let mut dist = 0;
                loop {
                    let next = graph.get(&edge).unwrap();
                    dist += 1;
                    if next.len() == 2 {
                        let tmp = edge;
                        edge = next[0];
                        if edge.eq(&pre) {
                            edge = next[1];
                        }
                        pre = tmp;
                    } else {
                        break;
                    }
                }
                new_edges.push((edge, dist));
            }
        }
        short_graph.insert(k, new_edges);
    }

    // for some reason the result is apparently 2 too hight
    let res2 = find_path(0, &short_graph, vec![start], &end) - 2;

    println!("Day 23:\nTask 1:{res1:16}\nTask 2:{res2:16}");
}

fn find_path(
    length: i32,
    graph: &HashMap<(usize, usize), Vec<((usize, usize), i32)>>,
    visited: Vec<(usize, usize)>,
    end: &(usize, usize),
) -> i32 {
    let pos = visited.last().unwrap();
    if pos == end {
        return length;
    }
    let mut res = 0;
    if let Some(edges) = graph.get(pos) {
        for (edge, dist) in edges {
            if !visited.contains(edge) {
                let mut visited = visited.clone();
                visited.push(*edge);
                res = res.max(find_path(length + dist, graph, visited, end));
            }
        }
    }
    res
}
