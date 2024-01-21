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
    ret
}

fn main() {
    println!("Hello, world!");
    let data = include_bytes!("../input.txt");
    let mut instructions = None;

    // Get the map size.
    let (m, n): (usize, usize);
    {
        let mut iter = data.split(|b| *b == b'\n');
        n = iter.next().unwrap().len();
        let mut counts = 1;
        let mut encountered_empty_line = false;
        for line in iter {
            if line.len() == 0 {
                encountered_empty_line = true;
            }
            if !encountered_empty_line {    
                counts += 1;
            }
            if encountered_empty_line && line.len() != 0 {
                instructions = Some(line);
                break;
            }
        }
        m = counts;
    }

    let mut map = ndarray::Array2::<u8>::zeros((m, n));
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
                        start_pos = Some((i, j));
                        break;
                    },
                    _ => { unreachable!(); }
                };
            }
        }
        for (j, c) in line.iter().enumerate() {
            map[[i, j]] = match c {
                b' ' => 0,
                b'#' => 1,
                b'.' => 2,
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

                    let fall_off_edge = 
                        next_i < 0 || next_i >= m as i32 || next_j < 0 || next_j >= n as i32 
                        || map[[next_i as usize, next_j as usize]] == 0;
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
                        1 => { /* Hit the wall */ break; },
                        2 => { /* Next is a valid tile */ },
                        _ => { unreachable!(); }
                    }
                    pos_i = next_i as usize;
                    pos_j = next_j as usize;
                }
            },
            Inst::Turn(turn) => {
                dir = (dir + turn + 4) % 4;
            },
        }
    }
    println!("{} {} {} {}", pos_i, pos_j, dir, pos_i * 1000 + 4 * pos_j + dir as usize);
}
