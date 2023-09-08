use std::collections::VecDeque;

fn main() {
    let mut canvas = ndarray::Array3::<u8>::zeros((100, 100, 100));
    for line in include_str!("../input.txt").lines() {
        let temp = line.split(",")
            .map(|num_str| num_str.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        canvas[[temp[0]+1, temp[1]+1, temp[2]+1]] = 1;
    }
    let mut count = 0;
    for i in 0..100 {
        for j in 0..100 {
            for k in 0..100 {
                if canvas[[i, j, k]] != 1 {
                    continue;
                }
                if i > 0 {
                    count += (canvas[[i-1, j, k]] != 1) as i32;
                }
                if i < 100-1 {
                    count += (canvas[[i+1, j, k]] != 1) as i32;
                }
                if j > 0 {
                    count += (canvas[[i, j-1, k]] != 1) as i32;
                }
                if j < 100-1 {
                    count += (canvas[[i, j+1, k]] != 1) as i32;
                }
                if k > 0 {
                    count += (canvas[[i, j, k-1]] != 1) as i32;
                }
                if k < 100-1 {
                    count += (canvas[[i, j, k+1]] != 1) as i32;
                }
            }
        }
    }
    println!("{}", count);
    println!("==============");

    let mut to_diffuse = VecDeque::<[usize;3]>::new();
    to_diffuse.push_back([0, 0, 0]);
    canvas[[0, 0, 0]] = 2;
    while let Some([i, j, k]) = to_diffuse.pop_front() {
        let mut ijk_s = Vec::<[usize; 3]>::new();
        if i > 0     { ijk_s.push([i-1, j, k]); }
        if i < 100-1 { ijk_s.push([i+1, j, k]); }
        if j > 0     { ijk_s.push([i, j-1, k]); }
        if j < 100-1 { ijk_s.push([i, j+1, k]); }
        if k > 0     { ijk_s.push([i, j, k-1]); }
        if k < 100-1 { ijk_s.push([i, j, k+1]); }

        for ijk in ijk_s {
            if canvas[ijk] == 0 {
                canvas[ijk] = 2;
                to_diffuse.push_back(ijk);
            }
        }
    }
    let mut count = 0;
    for i in 0..100 {
        for j in 0..100 {
            for k in 0..100 {
                if canvas[[i, j, k]] != 1 {
                    continue;
                }
                if i > 0 {
                    count += (canvas[[i-1, j, k]] == 2) as i32;
                }
                if i < 100-1 {
                    count += (canvas[[i+1, j, k]] == 2) as i32;
                }
                if j > 0 {
                    count += (canvas[[i, j-1, k]] == 2) as i32;
                }
                if j < 100-1 {
                    count += (canvas[[i, j+1, k]] == 2) as i32;
                }
                if k > 0 {
                    count += (canvas[[i, j, k-1]] == 2) as i32;
                }
                if k < 100-1 {
                    count += (canvas[[i, j, k+1]] == 2) as i32;
                }
            }
        }
    }
    println!("{}", count);
}
