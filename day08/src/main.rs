extern crate ndarray;
use either::Either;
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
        for i in 0..m {
            if forest_height[[i, j]] > max_buf[i] {
                max_buf[i] = forest_height[[i, j]];
                visible[[i, j]] = 1u8;
            }
        }
        for i in 0..m {
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

    {
        let mut sweep = |dir: i8, dim: u8| {
            /* 
            The major direction: We compare the height of trees in this direction.
            The minor direction: Parallel processing. Each element in this direction is independent.

            When dim is 0, we either sweep from top to bottom (dir==1) or bottom to top (dir==-1).
            When dim is 1, we either sweep from left to right (dir==1) or right to left (dir==-1).
            */
            let (n_major, n_minor) = if dim == 0 { (m, n) } else { (n, m) };
            let minor_iter = 0..n_minor;
            let mut major_iter = 
                if dir == 1 { Either::Left(0..n_major) } else { Either::Right((0..n_major).rev())};
            let get_ij = if dim == 0 {
                |i_major: usize, i_minor: usize| { [i_major, i_minor] }
            } else {
                |i_major: usize, i_minor: usize| { [i_minor, i_major] }
            };
            let mut dp_num_seen = Array2::<u32>::ones((n_minor, NUM_HEIGHTS));
            let first = major_iter.next().unwrap();
            score.index_axis_mut(Axis(dim as usize), first).fill(0);
            for i_major in major_iter {
                for i_minor in minor_iter.clone() {
                    let ij = get_ij(i_major, i_minor);
                    let height = forest_height[ij] as usize;
                    score[ij] *= dp_num_seen[[i_minor, height]];
                    for k in 0..(height+1) {
                        dp_num_seen[[i_minor, k]] = 1;
                    }
                    for k in (height+1)..NUM_HEIGHTS {
                        dp_num_seen[[i_minor, k]] += 1;
                    }
                }
            }
        };
        
        sweep(1, 0);
        sweep(-1, 0);
        sweep(1, 1);
        sweep(-1, 1);
    }

    let mut max = 0u32;
    score.for_each(|&v| { if v > max { max = v; }});
    println!("{}", max);
}
