extern crate ndarray;

use ndarray::s;

fn main() {
    let data = include_str!("../input.txt");
    let (mut ymax, mut xmin, mut xmax) = (0usize, 500i32, 500usize);
    for line in data.lines() {
        for coordinate_str in line.split(" -> ") {
            let (x_str, y_str) = coordinate_str.split_once(",").unwrap();
            let x: i32 = x_str.parse().unwrap();
            let y: usize = y_str.parse().unwrap();
            xmin = xmin.min(x);
            xmax = xmax.max(x as usize);
            ymax = ymax.max(y);
        }
    }
    if xmin < 0 { panic!("No supported"); }
    
    part1(data, ymax, xmax);
    println!("================");
    part2(data, ymax, xmax);
}


fn fill_grid(grid: &mut ndarray::Array2<u8>, data: &str) {
    // Fill the grid with rock positions
    for line in data.lines() {
        let mut prev_xy: Option<(usize, usize)> = None;
        for coordinate_str in line.split(" -> ") {
            let (x_str, y_str) = coordinate_str.split_once(",").unwrap();
            let x: usize = x_str.parse().unwrap();
            let y: usize = y_str.parse().unwrap();
            match prev_xy {
                None => {},
                Some((x_prev, y_prev)) => {
                    let (xmin, xmax) = (x.min(x_prev), x.max(x_prev));
                    let (ymin, ymax) = (y.min(y_prev), y.max(y_prev));
                    grid.slice_mut(s![ymin..=ymax, xmin..=xmax]).fill(1);
                },
            }
            prev_xy = Some((x, y));
        }
    }
}

fn part1(data: &str, ymax: usize, xmax: usize) {
    let height = ymax+1;
    let mut grid = ndarray::Array2::<u8>::zeros((height, xmax+1+1));

    fill_grid(&mut grid, data);

    let mut sand_count = 0;
    loop {
        let mut sand_x: usize = 500;
        let mut sand_y: usize = 0;
        if grid[[sand_y, sand_x]] != 0 {
            panic!("start point is stuck!");
        }

        while sand_y != height - 1 {
            if grid[[sand_y+1, sand_x]] == 0 {
                sand_y += 1;
            } else if grid[[sand_y+1, sand_x-1]] == 0 {
                sand_y += 1;
                sand_x -= 1;
            } else if grid[[sand_y+1, sand_x+1]] == 0 {
                sand_y += 1;
                sand_x += 1;
            } else {
                // Rest
                grid.slice_mut(s![sand_y, sand_x]).fill(1);
                break;
            }
        }

        let into_abyss = sand_y == height - 1;
        if into_abyss {
            break;
        }
        sand_count += 1;
    }

    println!("{}", sand_count);
}


fn part2(data: &str, ymax: usize, xmax: usize) {
    let height: usize = ymax + 2 + 1;  // +2 for the y coordinate of the floor.
    let width: usize;
    {
        // use a big enough buffer to avoid sand hitting right boundary of grid.
        let width_buf = 3usize;
        width = (height + width_buf + 500).max(xmax);
        if height + width_buf >= 500 {
            panic!("unsupported size!");
        }
    }
    let mut grid = ndarray::Array2::<u8>::zeros((height, width));
    fill_grid(&mut grid, data);

    grid.slice_mut(s![height-1, ..]).fill(1);

    let mut sand_count = 0;
    loop {
        let mut sand_x: usize = 500;
        let mut sand_y: usize = 0;
        if grid[[sand_y, sand_x]] != 0 {
            break;
        }

        while sand_y != height - 1 {
            if grid[[sand_y+1, sand_x]] == 0 {
                sand_y += 1;
            } else if grid[[sand_y+1, sand_x-1]] == 0 {
                sand_y += 1;
                sand_x -= 1;
            } else if grid[[sand_y+1, sand_x+1]] == 0 {
                sand_y += 1;
                sand_x += 1;
            } else {
                // Rest
                grid.slice_mut(s![sand_y, sand_x]).fill(1);
                break;
            }
        }

        let into_abyss = sand_y == height - 1;
        if into_abyss {
            unreachable!();
        }
        sand_count += 1;
    }

    println!("{}", sand_count);
}
