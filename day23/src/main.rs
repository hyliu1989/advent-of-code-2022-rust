extern crate ndarray;

use ndarray::{s, Axis};

#[derive(PartialEq)]
enum MapMark {
    None = 0,
    Conflict = 2,
    Unmoved = 9,
    FromW = 1,
    FromN = 3,
    FromE = 5,
    FromS = 7,
}

impl MapMark {
    fn from(i: u8) -> MapMark {
        match i {
            0 => MapMark::None,
            2 => MapMark::Conflict,
            9 => MapMark::Unmoved,
            1 => MapMark::FromW,
            3 => MapMark::FromN,
            5 => MapMark::FromE,
            7 => MapMark::FromS,
            _ => panic!("Invalid input"),
        }
    }
}


fn build_map(data: &[u8]) -> ndarray::Array2::<u8> {
    let m = 
        data.split(|b| *b == b'\n')
            .filter(|l| l.len() != 0)
            .count();
    let n = 
        data.split(|b| *b == b'\n')
            .next()
            .unwrap()
            .len();
    let mut map = ndarray::Array2::<u8>::zeros((m, n));
    for (i, line) in data.split(|b| *b == b'\n').enumerate() {
        for (j, c) in line.iter().enumerate() {
            map[[i, j]] = match c {
                b'.' => 0,
                b'#' => 1,
                _ => panic!("Invalid input"),
            }
        }
    }
    map
}


fn revert_1_location_map_next(map_next: &mut ndarray::Array2::<u8>, i: usize, j: usize) {
    let origin: MapMark = MapMark::from(map_next[[i, j]]);
    if origin == MapMark::Conflict {
        return;
    }
    let revert_loc = match origin {
        MapMark::FromW => { [i, j-1] },
        MapMark::FromN => { [i-1, j] },
        MapMark::FromE => { [i, j+1] },
        MapMark::FromS => { [i+1, j] },
        _ => panic!("Invalid input"),
    };
    assert!(map_next[revert_loc] == MapMark::None as u8);
    map_next[revert_loc] = MapMark::Unmoved as u8;
    map_next[[i, j]] = MapMark::Conflict as u8;
}


fn main() {
    println!("Hello, world!");
    let data = include_bytes!("../input.txt");
    let mut map = build_map(data);
    let (mut m, mut n) = map.dim();
    let mut expand_w: bool = true;
    let mut expand_e: bool = true;
    let mut expand_n: bool = true;
    let mut expand_s: bool = true;
    // let mut map2 = ndarray::Array2::<u8>::zeros((m+1, n+1));
    // map2.slice_mut(s![1.., 1..]).assign(&map);

    let mut iter_count = 0;
    let mut idx_case_start = 0;
    loop {
        if expand_w || expand_e || expand_n || expand_s {
            let m_new = m + if expand_n { 1 } else { 0 } + if expand_s { 1 } else { 0 };
            let n_new = n + if expand_w { 1 } else { 0 } + if expand_e { 1 } else { 0 };
            let mut map2 = ndarray::Array2::<u8>::zeros((m_new, n_new));
            let slice_y_start = if expand_n { 1 } else { 0 };
            let slice_x_start = if expand_w { 1 } else { 0 };
            map2.slice_mut(s![slice_y_start..slice_y_start+m, slice_x_start..slice_x_start+n]).assign(&map);
            map = map2;
            m = m_new;
            n = n_new;
            expand_w = false;
            expand_e = false;
            expand_n = false;
            expand_s = false;
        }

        // map_next will mark the next position of the elf.
        // The mark will indicate where the elf moves from. See MapMark enum for details.
        let mut no_one_moved = true;
        let mut map_next = ndarray::Array2::<u8>::zeros((m, n));
        for i in 0..m {
            for j in 0..n {
                if map[[i, j]] == 1 {
                    // Resolving where to go.
                    let mut loc_move_to = Option::<[usize; 2]>::None;
                    let mut on_x_of_destinations = Option::<MapMark>::None;
                    if map[[i-1, j-1]] == 0 && map[[i-1, j]] == 0 
                        && map[[i-1, j+1]] == 0 && map[[i, j+1]] == 0
                        && map[[i+1, j+1]] == 0 && map[[i+1, j]] == 0
                        && map[[i+1, j-1]] == 0 && map[[i, j-1]] == 0
                    {
                        // No need to move.
                    } else { 
                        for k in 0..4 {
                            let idx_case = (idx_case_start + k) % 4;
                            if idx_case == 0 { // Check North
                                if map[[i-1, j-1]] == 0 && map[[i-1, j]] == 0 && map[[i-1, j+1]] == 0 {
                                    loc_move_to = Some([i-1, j]);
                                    on_x_of_destinations = Some(MapMark::FromS);
                                    break;
                                }
                            } else if idx_case == 1 {  // Check South
                                if map[[i+1, j-1]] == 0 && map[[i+1, j]] == 0 && map[[i+1, j+1]] == 0 {
                                    loc_move_to = Some([i+1, j]);
                                    on_x_of_destinations = Some(MapMark::FromN);
                                    break;
                                }
                            } else if idx_case == 2 {  // Check West
                                if map[[i-1, j-1]] == 0 && map[[i, j-1]] == 0 && map[[i+1, j-1]] == 0 {
                                    loc_move_to = Some([i, j-1]);
                                    on_x_of_destinations = Some(MapMark::FromE);
                                    break;
                                }
                            } else if idx_case == 3 {  // Check East
                                if map[[i-1, j+1]] == 0 && map[[i, j+1]] == 0 && map[[i+1, j+1]] == 0 {
                                    loc_move_to = Some([i, j+1]);
                                    on_x_of_destinations = Some(MapMark::FromW);
                                    break;
                                }
                            }
                        }
                        no_one_moved = false;
                    }

                    if let Some(loc_move_to) = loc_move_to {
                        // There is a place to move to.
                        if map_next[loc_move_to] != MapMark::None as u8 {
                            // There is already an elf moving to the same place.
                            revert_1_location_map_next(&mut map_next, loc_move_to[0], loc_move_to[1]);
                            assert!(map_next[loc_move_to] == MapMark::Conflict as u8);
                            map_next[[i, j]] = MapMark::Unmoved as u8;
                        } else {
                            map_next[loc_move_to] = on_x_of_destinations.unwrap() as u8;
                            assert!(map_next[[i, j]] == MapMark::None as u8);
                        }
                    } else {
                        // No place to move.
                        map_next[[i, j]] = MapMark::Unmoved as u8;
                    }
                }
            }
        }
        // Assign the map_next to map.
        for i in 0..m {
            for j in 0..n {
                map[[i, j]] = map_next[[i, j]] % 2;
                if j == 0 && map[[i, j]] == 1 {
                    expand_w = true;
                }
                if j == n-1 && map[[i, j]] == 1 {
                    expand_e = true;
                }
                if i == 0 && map[[i, j]] == 1 {
                    expand_n = true;
                }
                if i == m-1 && map[[i, j]] == 1 {
                    expand_s = true;
                }
            }
        }

        idx_case_start = (idx_case_start + 1) % 4;
        iter_count += 1;

        if no_one_moved {
            println!("part2: {}", iter_count);
            break;
        }

        if iter_count == 10 {
            // Calculate the score
            let map = map.mapv(|x| u32::from(x));  // astype(uint32)
            let sum_row = map.sum_axis(Axis(0));
            let sum_col = map.sum_axis(Axis(1));
            let mut empty_cols: usize = 0;
            let mut empty_rows: usize = 0;
            for j in 0..n {
                if sum_row[j] != 0 { break; }
                empty_cols += 1;
            }
            for j in (0..n).rev() {
                if sum_row[j] != 0 { break; }
                empty_cols += 1;
            }
            for i in (0..m).rev() {
                if sum_col[i] != 0 { break; };
                empty_rows += 1;
            }
            for i in (0..m).rev() {
                if sum_col[i] != 0 { break; };
                empty_rows += 1;
            }
            let m_trimmed = m - empty_rows;
            let n_trimmed = n - empty_cols;
            let num_elves = sum_col.sum();
            assert!(num_elves == sum_row.sum());
            println!("part1: {}", m_trimmed * n_trimmed - num_elves as usize);
        }
    }
}
