extern crate ndarray;


enum Inst {
    Move(usize),
    Turn(i8),
}


fn parse_instruction(instructions: &[u8]) -> Vec<Inst> {
    let mut ret = Vec::new();
    let mut curr = 0;
    for c in instructions {
        match c {
            b'0'..=b'9' => {
                curr *= 10;
                curr += (c - b'0') as usize;
            },
            b'R' => {
                ret.push(Inst::Move(curr));
                ret.push(Inst::Turn(1));
                curr = 0;
            },
            b'L' => {
                ret.push(Inst::Move(curr));
                ret.push(Inst::Turn(-1));
                curr = 0;
            },
            _ => { unreachable!(); }
        }
    }
    if curr != 0 {
        ret.push(Inst::Move(curr));
    }
    ret
}


fn dir_to_delta(dir: i8) -> (i32, i32) {
    match dir {
        0 => (0, 1),
        1 => (1, 0),
        2 => (0, -1),
        3 => (-1, 0),
        _ => { unreachable!(); }
    }
}


fn fall_off_tranport_v1(map: &ndarray::Array2<u8>, current_pos: (usize, usize), dir: i8) -> (i32, i32, i8) {
    let (m, n) = map.dim();
    let (delta_i, delta_j) = dir_to_delta(dir.clone());
    let mut next_i: i32 = current_pos.0 as i32 + delta_i;
    let mut next_j: i32 = current_pos.1 as i32 + delta_j;
    if delta_i != 0 {
        next_i = if delta_i == 1 { 0 } else { m as i32 - 1 };
        while map[[next_i as usize, next_j as usize]] == 0 {
            next_i += delta_i;
        }
    } else if delta_j != 0 {
        next_j = if delta_j == 1 { 0 } else { n as i32 - 1 };
        while map[[next_i as usize, next_j as usize]] == 0 {
            next_j += delta_j;
        }
    }
    (next_i, next_j, dir)
}


fn fall_off_tranport_v2(map: &ndarray::Array2<u8>, current_pos: (usize, usize), dir: i8) -> (i32, i32, i8) {
    /*
             B    F
          +----+----+
         G|    |    |E
          |    |    |
          +----+----+
         A|    |  D
       A  |    |D
     +----+----+
    G|    |    |E
     |    |    |
     +----+----+
    B|    |  C
     |    |C
     +----+
       F
    
    */
    let (pos_i, pos_j) = current_pos;
    let next_i: i32;
    let next_j: i32;
    let next_dir: i8;

    if pos_i == 1 && dir == 3 {  // Start of horizontal edge conditions
        if 51 <= pos_j && pos_j <= 100 {
            // B
            next_i = (pos_j as i32 - 51) + 151;
            next_j = 1;
            next_dir = 0;
        } else if 101 <= pos_j && pos_j <= 150 {
            // F
            next_i = 200;
            next_j = (pos_j as i32 - 101) + 1;
            next_dir = 3;
        } else {
            unreachable!();
        }
    } else if pos_i == 50 && dir == 1 {
        if 101 <= pos_j && pos_j <= 150 {
            // D
            next_i = (pos_j as i32 - 101) + 51;
            next_j = 100;
            next_dir = 2;
        } else {
            unreachable!();
        }
    } else if pos_i == 101 && dir == 3 {
        if 1 <= pos_j && pos_j <= 50 {
            // A
            next_i = (pos_j as i32 - 1) + 51;
            next_j = 51;
            next_dir = 0;
        } else {
            unreachable!();
        }
    } else if pos_i == 150 && dir == 1 {
        if 51 <= pos_j && pos_j <= 100 {
            // C
            next_i = (pos_j as i32 - 51) + 151;
            next_j = 50;
            next_dir = 2;
        } else {
            unreachable!();
        }
    } else if pos_i == 200 && dir == 1 {
        if 1 <= pos_j && pos_j <= 50 {
            // F
            next_i = 1;
            next_j = (pos_j as i32 - 1) + 101;
            next_dir = 1;
        } else {
            unreachable!();
        }
    } else if pos_j == 1 && dir == 2 {   // Start of vertical edge conditions
        if 101 <= pos_i && pos_i <= 150 {
            // G
            next_i = 50 - (pos_i as i32 - 101);
            next_j = 51;
            next_dir = 0;
        } else if 151 <= pos_i && pos_i <= 200 {
            // B
            next_i = 1;
            next_j = (pos_i as i32 - 151) + 51;
            next_dir = 1;
        } else {
            unreachable!();
        }
    } else if pos_j == 50 && dir == 0 {
        if 151 <= pos_i && pos_i <= 200 {
            // C
            next_i = 150;
            next_j = (pos_i as i32 - 151) + 51;
            next_dir = 3;
        } else {
            unreachable!();
        }
    } else if pos_j == 51 && dir == 2 {
        if 1 <= pos_i && pos_i <= 50 {
            // G
            next_i = 150 - (pos_i as i32 - 1);
            next_j = 1;
            next_dir = 0;
        } else if 51 <= pos_i && pos_i <= 100 {
            // A
            next_i = 101;
            next_j = (pos_i as i32 - 51) + 1;
            next_dir = 1;
        } else {
            unreachable!();
        }
    } else if pos_j == 100 && dir == 0 {
        if 51 <= pos_i && pos_i <= 100 {
            // D
            next_i = 50;
            next_j = (pos_i as i32 - 51) + 101;
            next_dir = 3;
        } else if 101 <= pos_i && pos_i <= 150 {
            // E
            next_i = 50 - (pos_i as i32 - 101);
            next_j = 150;
            next_dir = 2;
        } else {
            unreachable!();
        }
    } else if pos_j == 150 && dir == 0 {
        if 1 <= pos_i && pos_i <= 50 {
            // E
            next_i = 150 - (pos_i as i32 - 1);
            next_j = 100;
            next_dir = 2;
        } else {
            unreachable!();
        }
    } else {
        unreachable!("{} {} {}", pos_i, pos_j, dir);
    }
    match next_dir {
        0 => { assert!(next_j % 2 == 1); },
        1 => { assert!(next_i % 2 == 1); },
        2 => { assert!(next_j % 2 == 0); },
        3 => { assert!(next_i % 2 == 0); },
        _ => { unreachable!(); }
    }

    (next_i, next_j, next_dir)
}


fn task_move(map: &ndarray::Array2<u8>, current_pos: (usize, usize), mut dir: i8, steps: usize, version: u8) -> (usize, usize, i8) {
    let (m, n) = map.dim();
    let (mut pos_i, mut pos_j) = current_pos;

    for _ in 0..steps {
        let (delta_i, delta_j) = dir_to_delta(dir.clone());
        let mut next_i: i32 = pos_i as i32 + delta_i;
        let mut next_j: i32 = pos_j as i32 + delta_j;
        let mut next_dir: i8 = dir;

        // Padded map should not lead to next_i or next_j being out of boundary.
        let error_happen = next_i < 0 || next_i >= m as i32 || next_j < 0 || next_j >= n as i32;
        assert!(!error_happen);
        let fall_off_edge = map[[next_i as usize, next_j as usize]] == 0;
        if fall_off_edge {
            (next_i, next_j, next_dir) = if version == 1 {
                fall_off_tranport_v1(&map, (pos_i.clone(), pos_j.clone()), dir.clone())
            } else {
                fall_off_tranport_v2(&map, (pos_i.clone(), pos_j.clone()), dir.clone())
            }
        }
        match map[[next_i as usize, next_j as usize]] {
            b'#' => { /* Hit the wall */ break; },
            b'.' => { /* Next is a valid tile */ },
            _ => { unreachable!(); }
        }
        pos_i = next_i as usize;
        pos_j = next_j as usize;
        dir = next_dir;
    }

    (pos_i, pos_j, dir)
}


fn main() {
    println!("Hello, world!");
    let data = include_bytes!("../input.txt");
    let mut instructions = None;

    // Get the map size.
    let (m, n): (usize, usize);
    {
        let mut encountered_empty_line = false;
        let mut line_len_max: i32 = 0;
        m = data
            .split(|b| *b == b'\n')
            .filter_map(move |l| {
                encountered_empty_line |= l.len() == 0;
                if encountered_empty_line { None } else { Some(l) }
            })
            .map(|l| { line_len_max = std::cmp::max(line_len_max, l.len() as i32); }) // side effect used!
            .count();
        n = line_len_max as usize;

        let mut encountered_empty_line = false;
        data
            .split(|b| *b == b'\n')
            .filter_map(move |l| {
                encountered_empty_line |= l.len() == 0;
                if encountered_empty_line && l.len() != 0 { Some(l) } else { None }
            })
            .for_each(|l| { instructions = Some(l); });
    }

    let mut map = ndarray::Array2::<u8>::zeros((m+2, n+2));
    let mut start_pos = None;
    for (i, line) in data.split(|b| *b == b'\n').enumerate() {
        if line.len() == 0 {
            break;
        }
        if i == 0 {
            for (j, c) in line.iter().enumerate() {
                match c {
                    b' ' | b'#' => {},
                    b'.' => {
                        start_pos = Some((i+1, j+1));
                        break;
                    },
                    _ => { unreachable!(); }
                };
            }
        }
        for (j, c) in line.iter().enumerate() {
            // Padding the map with empty tiles on the four edges.
            map[[i+1, j+1]] = match c {
                b' ' => 0,
                b'#' | b'.' => *c,
                _ => { unreachable!(); }
            };
        }
    }

    let instructions = parse_instruction(instructions.unwrap());
    let (mut pos_i, mut pos_j) = start_pos.unwrap();
    let mut dir = 0;
    
    for inst in instructions {
        match inst {
            Inst::Move(steps) => {
                (pos_i, pos_j, dir) = task_move(&map, (pos_i.clone(), pos_j.clone()), dir.clone(), steps, 1);
            },
            Inst::Turn(turn) => {
                dir = (dir + turn + 4) % 4;
            },
        }
    }
    println!("{} {} {} {}", pos_i, pos_j, dir, 1000 * (pos_i as i32) + 4 * (pos_j as i32) + (dir as i32));
}
