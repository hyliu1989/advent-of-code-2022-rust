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
    
    part1(data, ymax+1, xmax+1+1);
    println!("Hello, world!");
}

fn part1(data: &str, height: usize, width: usize) {
    let mut grid = ndarray::Array2::<u8>::zeros((height, width));

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

