extern crate ndarray;
use ndarray::prelude::*;


const NUM_HEIGHTS: usize = 10usize;

fn main() {
    let data = include_bytes!("../input.txt");
    part1(data);
    println!("Hello, world!");
    part2(data);
}

fn get_dims(data: &[u8]) -> (usize, usize) {
    let mut iter = data.split(|b| *b == b'\n');
    let first_line = iter.next().unwrap();
    (
        iter.filter(|&l| {l.len() != 0}).count() + 1,
        first_line.len(),
    )
}

fn build_forest_height(data: &[u8]) ->Array2<u8> {
    let (m, n) = get_dims(data);
    let mut forest_height = Array2::<u8>::zeros((m, n));
    let mut i = 0usize;
    for line in data.split(|b| {*b == b'\n'}) {
        if line.len() == 0 {
            break;
        }
        let mut j = 0usize;
        for c in line {
            forest_height[[i, j]] = c - b'0';
            j += 1;
        }
        i += 1;
    }
    forest_height
}

fn part1(data: &[u8]) {
    let forest_height = build_forest_height(data);
    let (m, n) = forest_height.dim();
    println!("dimensions: ({}, {})", m, n);
    let mut visible = Array2::<u8>::zeros((m, n));
    
    visible.slice_mut(s![0, ..]).fill(1);
    let mut max_buf = forest_height.slice(s![0, ..]).to_owned();
    visible.slice_mut(s![m-1, ..]).fill(1);
    let mut max_buf_bot = forest_height.slice(s![m-1, ..]).to_owned();
    for i in 1..m {
        for j in 0..n {
            if forest_height[[i, j]] > max_buf[j] {
                max_buf[j] = forest_height[[i, j]];
                visible[[i, j]] = 1u8;
            }
        }
        for j in 0..n {
            if forest_height[[m-1-i, j]] > max_buf_bot[j] {
                max_buf_bot[j] = forest_height[[m-1-i, j]];
                visible[[m-1-i, j]] = 1u8;
            }
        }
    }

    visible.slice_mut(s![.., 0]).fill(1);
    let mut max_buf = forest_height.slice(s![.., 0]).to_owned();
    visible.slice_mut(s![.., n-1]).fill(1);
    let mut max_buf_right = forest_height.slice(s![.., n-1]).to_owned();
    for j in 1..n {
        for i in 0..n {
            if forest_height[[i, j]] > max_buf[i] {
                max_buf[i] = forest_height[[i, j]];
                visible[[i, j]] = 1u8;
            }
        }
        for i in 0..n {
            if forest_height[[i, n-1-j]] > max_buf_right[i] {
                max_buf_right[i] = forest_height[[i, n-1-j]];
                visible[[i, n-1-j]] = 1u8;
            }
        }
    }
    
    let mut counts = 0;
    visible.for_each(|el| { if *el != 0 {counts += 1} });
    println!("{}", counts);
}


fn part2(data: &[u8]) {
    let forest_height = build_forest_height(data);
    let (m, n) = forest_height.dim();
    let mut score = Array2::<u32>::ones((m, n));

    let mut dp_num_seen = Array2::<u32>::ones((n, NUM_HEIGHTS));
    score.slice_mut(s![0, ..]).fill(0);
    for i in 1..m {
        for j in 0..n {
            let height = forest_height[[i, j]] as usize;
            score[[i, j]] *= dp_num_seen[[j, height]];
            for k in 0..(height+1) {
                dp_num_seen[[j, k]] = 1;
            }
            for k in (height+1)..NUM_HEIGHTS {
                dp_num_seen[[j, k]] += 1;
            }
        }
    }
    let mut dp_num_seen = Array2::<u32>::ones((n, NUM_HEIGHTS));
    score.slice_mut(s![m-1, ..]).fill(0);
    for i in (0..m-1).rev() {
        for j in 0..n {
            let height = forest_height[[i, j]] as usize;
            score[[i, j]] *= dp_num_seen[[j, height]];
            for k in 0..(height+1) {
                dp_num_seen[[j, k]] = 1;
            }
            for k in (height+1)..NUM_HEIGHTS {
                dp_num_seen[[j, k]] += 1;
            }
        }
    }
    let mut dp_num_seen = Array2::<u32>::ones((m, NUM_HEIGHTS));
    score.slice_mut(s![.., 0]).fill(0);
    for j in 1..n {
        for i in 0..m {
            let height = forest_height[[i, j]] as usize;
            score[[i, j]] *= dp_num_seen[[i, height]];
            for k in 0..(height+1) {
                dp_num_seen[[i, k]] = 1;
            }
            for k in (height+1)..NUM_HEIGHTS {
                dp_num_seen[[i, k]] += 1;
            }
        }
    }
    let mut dp_num_seen = Array2::<u32>::ones((m, NUM_HEIGHTS));
    score.slice_mut(s![.., n-1]).fill(0);
    for j in (0..n-1).rev() {
        for i in 0..m {
            let height = forest_height[[i, j]] as usize;
            score[[i, j]] *= dp_num_seen[[i, height]];
            for k in 0..(height+1) {
                dp_num_seen[[i, k]] = 1;
            }
            for k in (height+1)..NUM_HEIGHTS {
                dp_num_seen[[i, k]] += 1;
            }
        }
    }

    let mut max = 0u32;
    score.for_each(|&v| { if v > max { max = v; }});
    println!("{}", max);
}
