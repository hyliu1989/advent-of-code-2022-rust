use std::collections::VecDeque;

extern crate ndarray;


fn build_map(data: &[u8]) -> ((usize, usize), (usize, usize), ndarray::Array2<u8>) {
    let (m, n): (usize, usize);
    {
        let mut iter = data.split(|b| *b == b'\n');
        n = iter.next().unwrap().len();
        m = iter.count() + 1;
    }
    let mut start_pos = (0usize, 0usize);
    let mut end_pos = (0usize, 0usize);
    let mut map = ndarray::Array2::<u8>::zeros((m, n));
    for (i, row) in data.split(|b| *b == b'\n').enumerate() {
        for (j, c) in row.iter().enumerate() {
            map[[i, j]] = match c {
                b'S' => { start_pos = (i, j); b'a' },
                b'E' => { end_pos = (i, j); b'z' },
                any => {*any},
            } - b'a' + 1
        }
    }

    (start_pos, end_pos, map)
}

fn run_bfs(mut queue: VecDeque::<[usize;2]>, mut visited: ndarray::Array2<u32>,
           map: ndarray::Array2<u8>, end_pos: (usize, usize)) -> i32 {
    let (m, n) = map.dim();
    while let Some(curr) = queue.pop_front() {
        let step_to_here = visited[curr];
        if curr == [end_pos.0, end_pos.1] {
            // subtract 1 because the start position has count 1 instead of 0.
            return (step_to_here - 1) as i32
        }
        let max_reach = map[curr] + 1;
        let mut neighbors: Vec<[usize; 2]> = Vec::new();
        if curr[0] != 0   { neighbors.push([curr[0]-1, curr[1]]); }
        if curr[0] != m-1 { neighbors.push([curr[0]+1, curr[1]]); }
        if curr[1] != 0   { neighbors.push([curr[0], curr[1]-1]); }
        if curr[1] != n-1 { neighbors.push([curr[0], curr[1]+1]); }
        for neighbor in neighbors {
            if visited[neighbor] != 0 { continue; }
            if map[neighbor] <= max_reach {
                visited[neighbor] = step_to_here + 1;
                queue.push_back(neighbor);
            }
        }
    }
    -1
}

fn part1(data: &[u8]) {
    let (start_pos, end_pos, map) = build_map(data);
    let mut visited = ndarray::Array2::<u32>::zeros(map.dim());
    let mut bfs: VecDeque::<[usize;2]> = VecDeque::new();
    visited[[start_pos.0, start_pos.1]] = 1;
    bfs.push_back([start_pos.0, start_pos.1]);
    let ret = run_bfs(bfs, visited, map, end_pos);
    println!("{}", ret);
}

fn part2(data: &[u8]) {
    let (_, end_pos, map) = build_map(data);
    let mut visited = ndarray::Array2::<u32>::zeros(map.dim());
    let mut bfs: VecDeque::<[usize;2]> = VecDeque::new();
    for ((i, j), el) in map.indexed_iter() {
        if *el == 1 {
            visited[[i, j]] = 1;
            bfs.push_back([i, j]);
        }
    }
    let ret = run_bfs(bfs, visited, map, end_pos);
    println!("{}", ret);
}

fn main() {
    let data = include_bytes!("../input.txt");
    part1(data);
    println!("=========================");
    part2(data);
}
