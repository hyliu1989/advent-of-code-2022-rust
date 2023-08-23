fn main() {
    part1();
    println!("==========");
    part2();
}

fn parse_instruction(line: &str) -> (u32, usize, usize) {
    let (num_inst_str, loc_inst_str) = line.split_once("from").unwrap();

    let (_, num) = num_inst_str.split_once("move ").unwrap();
    let (src, des) = loc_inst_str.split_once("to").unwrap();

    (
        num.trim().parse::<u32>().unwrap(),
        src.trim().parse::<usize>().unwrap() - 1usize,
        des.trim().parse::<usize>().unwrap() - 1usize,
    )
}

fn parse_stack_state(state_str: &str) -> Vec<Vec<char>> {
    let mut iter = state_str.rsplit("\n");
    let first_line = iter.next().unwrap();
    let n_stacks = ((first_line.chars().count() as f64) / 4.0).ceil() as usize;

    let mut ret = Vec::<Vec<char>>::with_capacity(n_stacks);
    for _ in 0..n_stacks {
        ret.push(Vec::<char>::new());
    }

    for line in iter {
        for (i, crate_str) in line.chars().skip(1).step_by(4).enumerate()
        {
            if crate_str != ' ' {
                ret[i].push(crate_str);
            }
        }
    }
    ret
}

fn part1() {
    let data = include_str!("../input.txt");
    let (init_state, instructions) = data.split_once("\n\n").unwrap();
    let mut stack = parse_stack_state(init_state);
    for line in instructions.lines() {
        let (num, src, des) = parse_instruction(line);
        for _ in 0..num {
            match stack[src].pop() {
                Some(crate_char) => stack[des].push(crate_char),
                None => { panic!("Error state!") },
            }
        }
    }
    let mut ret = String::with_capacity(stack.len());
    for i in 0..stack.len() {
        match stack[i].pop() {
            Some(top_crate_char) => ret.push(top_crate_char),
            None => {panic!("Empty stack {}", i)},
        }
    }
    println!("{}", ret);
}

fn part2() {
    let data = include_str!("../input.txt");
    let (init_state, instructions) = data.split_once("\n\n").unwrap();
    let mut stack = parse_stack_state(init_state);
    for line in instructions.lines() {
        let (num, src, des) = parse_instruction(line);
        
        let new_src_len = stack[src].len() - (num as usize);
        
        // Because of Rust does not allow borrowing `stack` as immutable and mutable at the same time,
        // I have to clone `stack[src].split_at(new_src_len).1` in order to operate on its content.
        // This prevents critical error that `src` and `des` happens to be the same index.
        let moved = Vec::<char>::from(stack[src].split_at(new_src_len).1);
        
        // Do the moving
        stack[des].extend_from_slice(&moved);
        stack[src].truncate(new_src_len);
    }
    let mut ret = String::with_capacity(stack.len());
    for i in 0..stack.len() {
        match stack[i].pop() {
            Some(top_crate_char) => ret.push(top_crate_char),
            None => {panic!("Empty stack {}", i)},
        }
    }
    println!("{}", ret);
}
