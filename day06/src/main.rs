fn main() {
    part1();
    println!("==========");
    part2();
}


const SIGNAL_START_LEN: usize = 4;
fn check_signal_start(l: &Vec<char>) -> usize {
    if l.len() < SIGNAL_START_LEN {
        return 0;
    }
    let mut neq_state = [true; SIGNAL_START_LEN];
    let get_state_arr_idx = |i: usize| { i % SIGNAL_START_LEN };

    let compare_and_record = |i_char: usize, prev_start: usize, neq_state: &mut [bool; SIGNAL_START_LEN]| {
        neq_state[get_state_arr_idx(i_char)] = true;
        for i_prev in prev_start..i_char {
            let i_buf = get_state_arr_idx(i_prev);
            neq_state[i_buf] = neq_state[i_buf] && (l[i_prev] != l[i_char]);
        }
    };
    let all = |neq_state: [bool; SIGNAL_START_LEN]| {
        (0..SIGNAL_START_LEN).all(|i: usize| neq_state[i])
    };
    
    // Build the buffer and check for the first 4 characters.
    for i in 0..SIGNAL_START_LEN {
        compare_and_record(i, 0, &mut neq_state);
    }
    if all(neq_state) {
        return SIGNAL_START_LEN;
    }

    // Sliding through the rest of the characters.
    for i in SIGNAL_START_LEN..l.len() {
        compare_and_record(i, i-(SIGNAL_START_LEN-1), &mut neq_state);
        if all(neq_state) {
            return i + 1;
        }
    }
    0
}

fn part1() {
    for l in include_str!("../input.txt").lines() {
        println!("{}", check_signal_start(&(l.chars().collect::<Vec<char>>())));
    }
}

fn part2() {

}
