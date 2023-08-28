use std::collections::HashSet;


struct Knot { i: i32, j: i32, trace: Option<HashSet<(i32, i32)>> }

impl Knot {
    fn new(i:i32, j:i32, mut trace: Option<HashSet<(i32, i32)>>) -> Self {
        if let Some(ref mut trace_map) = trace {
            trace_map.insert((i, j));
        }
        Knot {i, j, trace}
    }
    fn update(&mut self, head_i: i32, head_j: i32) {
        /* Update the tail position with given head position. */
        let mut i_shifted = false;
        let mut j_shifted = false;
        if (head_i - self.i) >= 2 {
            self.i = head_i - 1;
            i_shifted = true;
        } else if (head_i - self.i) <= -2 {
            self.i = head_i + 1;
            i_shifted = true;
        }
        if (head_j - self.j) >= 2 {
            self.j = head_j - 1;
            j_shifted = true;
        } else if (head_j - self.j) <= -2 {
            self.j = head_j + 1;
            j_shifted = true;
        }
        if i_shifted && !j_shifted {
            self.j = head_j;
        }
        if j_shifted && !i_shifted {
            self.i = head_i;
        }
        if !(i_shifted || j_shifted) {
            return;
        }
        if let Some(ref mut trace) = self.trace {
            trace.insert((self.i, self.j));
        }
    }
}

impl Default for Knot {
    fn default() -> Self { Knot {i: 0, j: 0, trace: None} }
}

fn main() {
    let data = include_str!("../input.txt");
    part1(data);
    println!("==============");
    // part2(include_str!("../input_small.txt"));
    part2(data);
}

fn part1(data: &str) {
    let (mut i, mut j) = (0i32, 0i32);  // Head position
    let (mut ii, mut jj) = (0i32, 0i32);  // Tail position
    let mut tail_trace: HashSet<(i32, i32)> = HashSet::new();
    tail_trace.insert((0, 0));
    // Parallelly using Knot struct to run the same thing to verify Knot implementation.
    let mut tail = Knot::new(0, 0, Some(HashSet::new()));

    {
        let mut update_tail = |i:i32, j: i32| {
            /* Update the tail position with given head position. */
            let mut changed: bool = false;
            if (i - ii) >= 2 {
                ii = i - 1;
                jj = j;
                changed = true;
            } else if (i - ii) <= -2 {
                ii = i + 1;
                jj = j;
                changed = true;
            }
            if (j - jj) >= 2 {
                ii = i;
                jj = j - 1;
                changed = true;
            } else if (j - jj) <= -2 {
                ii = i;
                jj = j + 1;
                changed = true;
            }
            if changed {
                tail_trace.insert((ii, jj));
            }
        };
        for line in data.lines() {
            let (direction, num_steps) = line.split_once(' ').unwrap();
            let num_steps: i32 = num_steps.parse().unwrap();
            let (delta_i, delta_j) = match direction {
                "U" => { (-1,  0) },
                "D" => { ( 1,  0) },
                "L" => { ( 0, -1) },
                "R"|_ => { (0, 1) },
            };
            for _ in 0..num_steps {
                i += delta_i;
                j += delta_j;
                update_tail(i, j);
                tail.update(i, j)
            }
        }
    }

    println!("{}", tail_trace.len());
    println!("{}", tail.trace.unwrap().len());
}


fn part2(data: &str) {
    let (mut i, mut j) = (0i32, 0i32);  // Head position
    let mut tails:[Knot; 9] = Default::default();
    tails[8] = Knot::new(0, 0, Some(HashSet::new()));

    for line in data.lines() {
        let (direction, num_steps) = line.split_once(' ').unwrap();
        let num_steps: i32 = num_steps.parse().unwrap();
        let (delta_i, delta_j) = match direction {
            "U" => { (-1,  0) },
            "D" => { ( 1,  0) },
            "L" => { ( 0, -1) },
            "R"|_ => { (0, 1) },
        };
        for _ in 0..num_steps {
            i += delta_i;
            j += delta_j;
            let mut ii = i;
            let mut jj = j;
            for idx in 0..9 {
                tails[idx].update(ii, jj);
                (ii, jj) = (tails[idx].i, tails[idx].j);
            }
        }

        // {
        //     // Debug print
        //     print!("({},{}) ", i, j);
        //     for idx in 0..9 {
        //         print!("({},{}) ", tails[idx].i, tails[idx].j);
        //     }
        //     println!("");
        // }
    }
    println!("{}", tails[8].trace.as_ref().unwrap().len());

    // {
    //     // debug print with input_small.txt
    //     let positions = tails[8].trace.as_ref().unwrap();
    //     let x_off = 20;
    //     const x_len: usize = 41;
    //     let y_off = 20;
    //     const y_len: usize= 41;
    //     let mut map = [[0u8; x_len]; y_len];
    //     for (i, j) in positions {
    //         map[(i+y_off) as usize][(j+x_off) as usize] = 1;
    //     }
    //     for i in 0..y_len {
    //         for j in 0..x_len {
    //             print!("{}", if map[i][j] == 1 { "#" } else { "." } );
    //         }
    //         println!("");
    //     }
    // }
}
