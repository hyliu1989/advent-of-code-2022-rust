use std::collections::HashSet;

fn main() {
    let data = include_str!("../input.txt");
    part1(data);
    println!("Hello, world!");
}

fn part1(data: &str) {
    let (mut i, mut j) = (0i32, 0i32);  // Head position
    let (mut ii, mut jj) = (0i32, 0i32);  // Tail position
    let mut tail_trace: HashSet<(i32, i32)> = HashSet::new();
    tail_trace.insert((0, 0));

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
            let mut delta_i = 0i32;
            let mut delta_j = 0i32;
            match direction {
                "U" => { delta_i = -1; },
                "D" => { delta_i =  1; },
                "L" => { delta_j = -1; },
                "R" => { delta_j =  1; },
                _ => {},
            }
            for _ in 0..num_steps {
                i += delta_i;
                j += delta_j;
                update_tail(i, j);
            }
        }
    }

    println!("{}", tail_trace.len());
}
