// Going upwards +y, going rightwards +x.
struct Rock {
    check_left: Vec<(i8, i8)>,  // (y, x) coordinates relative to the bottom-left most pixel
    check_right: Vec<(i8, i8)>,  // (y, x) coordinates relative to the bottom-left most pixel
    check_down: Vec<(i8, i8)>,  // (y, x) coordinates relative to the bottom-left most pixel
    fill: Vec<(i8, i8)>,
}

const DEBUG: bool = false;
const WIDTH: usize = 7;
// const NUM_ROCKS: usize = 2022;  // part 1
// const NUM_ROCKS: usize = 1_000_000_000_000;  // part 2
const NUM_ROCKS: usize = 256+304;  // part 2, after observing a repeating pattern.

fn main() {
    let rock_appearing_sequence = &[
        Rock {
            check_left: [(0, -1)].into(), 
            check_right: [(0, 4)].into(), 
            check_down: [(-1, 0), (-1, 1), (-1, 2), (-1, 3)].into(), 
            fill: [(0, 0), (0, 1), (0, 2), (0, 3)].into(),
        },
        Rock {
            check_left: [(0, 0), (1, -1), (2, 0)].into(), 
            check_right: [(0, 2), (1, 3), (2, 2)].into(), 
            check_down: [(0, 0), (-1, 1), (0, 2)].into(), 
            fill: [(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)].into(),
        },
        Rock {
            check_left: [(0, -1), (1, 1), (2, 1)].into(), 
            check_right: [(0, 3), (1, 3), (2, 3)].into(), 
            check_down: [(-1, 0), (-1, 1), (-1, 2)].into(), 
            fill: [(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)].into(),
        },
        Rock {
            check_left: [(0, -1), (1, -1), (2, -1), (3, -1)].into(), 
            check_right: [(0, 1), (1, 1), (2, 1), (3, 1)].into(), 
            check_down: [(-1, 0)].into(), 
            fill: [(0, 0), (1, 0), (2, 0), (3, 0)].into(),
        },
        Rock {
            check_left: [(0, -1), (1, -1)].into(), 
            check_right: [(0, 2), (1, 2)].into(), 
            check_down: [(-1, 0), (-1, 1)].into(), 
            fill: [(0, 0), (0, 1), (1, 0), (1, 1)].into(),
        },
    ];
    if DEBUG {
        for rock in rock_appearing_sequence {
            let mut canvas = ndarray::Array2::<u8>::zeros((7, 7));
            let (i_start, j_start) = (1i8, 1i8);
            for (rel_i, rel_j) in rock.check_down.iter() {
                canvas[[(i_start + rel_i) as usize, (j_start + rel_j) as usize]] = b'v';
            }
            for (rel_i, rel_j) in rock.check_left.iter() {
                canvas[[(i_start + rel_i) as usize, (j_start + rel_j) as usize]] = b'<';
            }
            for (rel_i, rel_j) in rock.check_right.iter() {
                canvas[[(i_start + rel_i) as usize, (j_start + rel_j) as usize]] = b'>';
            }
            for row in canvas.axis_iter(ndarray::Axis(0)).rev() {
                for c in row {
                    match c {
                        b'>' => print!(">"),
                        b'<' => print!("<"),
                        b'v' => print!("v"),
                        _ => print!("."),
                    }
                }
                println!("");
            }
            println!("===")
        }
        
    }

    let mut get_next_rock;
    {
        let mut idx_rock = 0usize;
        get_next_rock = move || { 
            let ret = &rock_appearing_sequence[idx_rock % rock_appearing_sequence.len()];
            idx_rock += 1;
            (ret, idx_rock-1)
        };
    }
    let mut remove_triggering_threshold = 500;
    let mut canvas: Vec<[bool; WIDTH]> = Vec::new();
    let mut removed_rows: usize = 0;
    let mut current_block_state: Option<(&Rock, i32, i8)> = None;
    let mut rock_idx = 0usize;
    for instruction in include_bytes!("../input.txt").into_iter().cycle() {
        let (rock, mut i, mut j);
        if let Some((rr, ii, jj)) = current_block_state {
            (i, j, rock) = (ii, jj, rr);
        } else {
            (i, j) = (canvas.len() as i32 + 3, 2);
            (rock, rock_idx) = get_next_rock();
            if rock_idx == NUM_ROCKS {
                break;
            }
            if DEBUG && rock_idx < 30 {
                print_canvas(&canvas);
                println!("=============");
            }
        }
        match instruction {
            b'>' => {
                let passed = check_shift(&canvas, &rock.check_right, i, j);
                if passed {
                    j += 1;
                }
            },
            b'<' => {
                let passed = check_shift(&canvas, &rock.check_left, i, j);
                if passed {
                    j -= 1;
                }
            },
            _ => { continue; }
        }
        let passed = check_shift(&canvas, &rock.check_down, i, j);
        if passed {
            i -= 1;
            current_block_state = Some((rock, i, j));
        } else {
            // Create the rock and extend the canvas if needed.
            for (rel_i, rel_j) in &rock.fill {
                let row_idx = (i + (*rel_i as i32)) as usize;
                while canvas.len() <= row_idx {
                    canvas.push([false; WIDTH]);
                }
                canvas[row_idx][(j + rel_j) as usize] = true;
            }
            // Delete canvas rows.
            // The criteria to show that a chunk of canvas is no longer needed is that the consecutive 4 rows contains
            // at least one element in every column.
            if canvas.len() > remove_triggering_threshold {
                const NUM_TEST_ROWS: usize = 1;
                let mut removed = false;
                for (i_rev_row, row_chunk) in canvas.windows(NUM_TEST_ROWS).rev().enumerate() {
                    if canvas.len() - i_rev_row - NUM_TEST_ROWS <= 0 {
                        break;
                    }
                    let mut ret = row_chunk[0].clone();
                    for i in 1..NUM_TEST_ROWS {
                        for j in 0..7 {
                            ret[j] |= row_chunk[i][j];
                        }
                    }
                    let blocking = {ret.into_iter().reduce(|acc, b| { acc & b })};
                    if blocking.unwrap() {
                        let removable_rows = canvas.len() as i32 - i_rev_row as i32 - NUM_TEST_ROWS as i32;
                        removed_rows += removable_rows as usize;
                        canvas.drain(0..(removable_rows as usize));
                        // Print out the rock id and the removed row height and use Excel to analyze the repeating pattern.
                        println!("{} {}", rock_idx, canvas.len() + removed_rows);
                        removed = true;
                        break;
                    }
                }
                if !removed {
                    remove_triggering_threshold += 500;
                } else {
                    remove_triggering_threshold = 500;
                }
            }
            current_block_state = None;
        }
    }
    println!("{}", canvas.len() + removed_rows);
}


fn check_shift(canvas: &Vec<[bool; WIDTH]>, relative_coordinates: &Vec<(i8, i8)>, i: i32, j: i8) -> bool {
    let mut passed = true;
    for (rel_i, rel_j) in relative_coordinates {
        let row_idx = i + (*rel_i as i32);
        if row_idx <= -1 {
            return false;
        }
        let col_idx = j + rel_j;
        if col_idx <= -1 || col_idx >= WIDTH as i8 {
            return false;
        }
        if let Some(row) = canvas.get(row_idx as usize) {
            let filled = row[col_idx as usize];
            if filled {
                passed = false;
                break;
            }
        }
    }
    passed
}


fn print_canvas(canvas: &Vec<[bool; WIDTH]>) {
    for row in canvas.into_iter().rev() {
        for c in row {
            print!( "{}", if *c { "@" } else {"."} );
        }
        println!("");
    }
}
