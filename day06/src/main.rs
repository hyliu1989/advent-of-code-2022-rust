fn main() {
    part1();
    println!("==========");
    part2();
}


fn check_distinct_seq(l: &Vec<char>, len_distinct: usize) -> usize {
    if l.len() < len_distinct {
        return 0;
    }
    let mut neq_state = vec![true; len_distinct];
    let get_state_arr_idx = |i: usize| { i % len_distinct };

    let compare_and_record = |i_char: usize, prev_start: usize, neq_state: &mut Vec<bool>| {
        neq_state[get_state_arr_idx(i_char)] = true;
        for i_prev in prev_start..i_char {
            let i_buf = get_state_arr_idx(i_prev);
            neq_state[i_buf] = neq_state[i_buf] && (l[i_prev] != l[i_char]);
        }
    };
    
    // Build the buffer and check for the first 4 characters.
    for i in 0..len_distinct {
        compare_and_record(i, 0, &mut neq_state);
    }
    if (0..len_distinct).all(|i| neq_state[i]) {
        return len_distinct;
    }

    // Sliding through the rest of the characters.
    for i in len_distinct..l.len() {
        compare_and_record(i, i-(len_distinct-1), &mut neq_state);
        if (0..len_distinct).all(|i| neq_state[i]) {
            return i + 1;
        }
    }
    0
}

fn check_signal_start(l: &Vec<char>) -> usize { check_distinct_seq(l, 4) }
fn check_message(l: &Vec<char>) -> usize { check_distinct_seq(l, 14) }

fn part1() {
    for l in include_str!("../input.txt").lines() {
        println!("{}", check_signal_start(&(l.chars().collect::<Vec<char>>())));
    }
}

fn part2() {
    for l in include_str!("../input.txt").lines() {
        println!("{}", check_message(&(l.chars().collect::<Vec<char>>())));
    }
}
