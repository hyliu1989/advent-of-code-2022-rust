fn main() {
    part1();
}


fn parse_line_part1(l: &str) -> bool {
    let (elf1_asmnt_str, elf2_asmnt_str) = l.split_once(',').unwrap();
    let (elf1_asmnt, elf2_asmnt) = (
        // Parse the first elf's assignment
        elf1_asmnt_str.split('-').map(|num_str| num_str.parse::<u32>().unwrap()).collect::<Vec<u32>>(),
        // Parse the second elf's assignment
        elf2_asmnt_str.split('-').map(|num_str| num_str.parse::<u32>().unwrap()).collect::<Vec<u32>>(),
    );
    let containing= |large: &Vec<u32>, small: &Vec<u32>| {
        large[0] <= small[0] && small[0] <= large[1] && large[0] <= small[1] && small[1] <= large[1]
    };
    containing(&elf1_asmnt, &elf2_asmnt) || containing(&elf2_asmnt, &elf1_asmnt)
}

fn part1() {
    let data = include_str!("../input.txt");
    let num = data.lines()
        .map(|l| { parse_line_part1(l) })
        .filter(|b| *b)
        .count();
    println!("{}", num);  // 466
}
