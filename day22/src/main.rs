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

fn task_move(map: &ndarray::Array2<u8>, current_pos: (usize, usize), dir: i8, steps: usize) -> (usize, usize) {
    let (m, n) = map.dim();
    let (mut pos_i, mut pos_j) = current_pos;
    let (delta_i, delta_j) = match dir {
        0 => (0, 1),
        1 => (1, 0),
        2 => (0, -1),
        3 => (-1, 0),
        _ => { unreachable!(); }
    };

    for _ in 0..steps {
        let mut next_i: i32 = pos_i as i32 + delta_i;
        let mut next_j: i32 = pos_j as i32 + delta_j;

        // Padded map should not lead to next_i or next_j being out of boundary.
        let error_happen = next_i < 0 || next_i >= m as i32 || next_j < 0 || next_j >= n as i32;
        assert!(!error_happen);
        let fall_off_edge = map[[next_i as usize, next_j as usize]] == 0;
        if fall_off_edge {
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
        }
        match map[[next_i as usize, next_j as usize]] {
            b'#' => { /* Hit the wall */ break; },
            b'.' => { /* Next is a valid tile */ },
            _ => { unreachable!(); }
        }
        pos_i = next_i as usize;
        pos_j = next_j as usize;
    }

    (pos_i, pos_j)
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
                (pos_i, pos_j) = task_move(&map, (pos_i.clone(), pos_j.clone()), dir.clone(), steps);
            },
            Inst::Turn(turn) => {
                dir = (dir + turn + 4) % 4;
            },
        }
    }
    println!("{} {} {} {}", pos_i, pos_j, dir, 1000 * (pos_i as i32) + 4 * (pos_j as i32) + (dir as i32));
}
