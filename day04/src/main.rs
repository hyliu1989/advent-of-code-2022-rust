fn main() {
    part1();
    println!("==========");
    part2();
}


fn parse_assignment_line(l: &str) -> (Vec<u32>, Vec<u32>) {
    let (elf1_asmnt_str, elf2_asmnt_str) = l.split_once(',').unwrap();
    let (elf1_asmnt, elf2_asmnt) = (
        // Parse the first elf's assignment
        elf1_asmnt_str.split('-').map(|num_str| num_str.parse::<u32>().unwrap()).collect::<Vec<u32>>(),
        // Parse the second elf's assignment
        elf2_asmnt_str.split('-').map(|num_str| num_str.parse::<u32>().unwrap()).collect::<Vec<u32>>(),
    );
    (elf1_asmnt, elf2_asmnt)
}

fn part1() {
    let containing= |large: &Vec<u32>, small: &Vec<u32>| {
        large[0] <= small[0] && small[0] <= large[1] && large[0] <= small[1] && small[1] <= large[1]
    };

    let data = include_str!("../input.txt");
    let num = data.lines()
        .map(|l| { parse_assignment_line(l) })
        .filter(|(elf1_asmnt, elf2_asmnt)| 
            containing(&elf1_asmnt, &elf2_asmnt) 
            || containing(&elf2_asmnt, &elf1_asmnt))
        .count();
    println!("{}", num);
}

fn part2() {
    let overlapped = |large: &Vec<u32>, small: &Vec<u32>| {
        (large[0] <= small[0] && small[0] <= large[1]) || (large[0] <= small[1] && small[1] <= large[1])
    };

    let data = include_str!("../input.txt");
    let num = data.lines()
        .map(|l| { parse_assignment_line(l) })
        .filter(|(elf1_asmnt, elf2_asmnt)| 
            overlapped(&elf1_asmnt, &elf2_asmnt) 
            || overlapped(&elf2_asmnt, &elf1_asmnt))
        .count();
    println!("{}", num);
}
