fn main() {
    part1();
}


fn parse_line_part1(l: &str) -> u32 {
    let elf_asmnt: Vec<Vec<u32>> = l.split(',')
        .map(|seg| {
            seg.split('-')
                .map(|num_str| {num_str.parse::<u32>().unwrap()})
                .collect::<Vec<u32>>()
        })
        .collect();
    let containing= |large: &Vec<u32>, small: &Vec<u32>| {
        large[0] <= small[0] && small[0] <= large[1] && large[0] <= small[1] && small[1] <= large[1]
    };
    if containing(&elf_asmnt[0], &elf_asmnt[1]) || containing(&elf_asmnt[1], &elf_asmnt[0]) {
        1u32
    } else {
        0u32
    }
}

fn part1() {
    let data = include_str!("../input.txt");
    let num = data.lines()
        .map(|l| { parse_line_part1(l) })
        .sum::<u32>();
    println!("{}", num);
}
