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
                if canvas[[i, j, k]] == 0 {
                    continue;
                }
                if i > 0 {
                    count += (canvas[[i-1, j, k]] == 0) as i32;
                }
                if i < 100-1 {
                    count += (canvas[[i+1, j, k]] == 0) as i32;
                }
                if j > 0 {
                    count += (canvas[[i, j-1, k]] == 0) as i32;
                }
                if j < 100-1 {
                    count += (canvas[[i, j+1, k]] == 0) as i32;
                }
                if k > 0 {
                    count += (canvas[[i, j, k-1]] == 0) as i32;
                }
                if k < 100-1 {
                    count += (canvas[[i, j, k+1]] == 0) as i32;
                } 
            }
        }
    }
    println!("{}", count);
    println!("Hello, world!");
}
