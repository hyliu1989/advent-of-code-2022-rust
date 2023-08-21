fn main() {
    let data = include_str!("../input.txt");
    let mut max = 0u32;
    for elf_str in data.split("\n\n") {
        let mut current_ration = 0u32;
        for ration_str in elf_str.lines() {
            println!("{}", ration_str);
            let ration = ration_str.parse::<u32>().unwrap();
            current_ration += ration;
        }
        if max < current_ration {
            max = current_ration;
        }
    }
    println!("{}", max);
}


/* from https://github.com/timvisee/advent-of-code-2022/blob/master/day01a/src/main.rs
pub fn main() {
    println!(
        "{}",
        include_str!("../input.txt")
            .split("\n\n")
            .map(|e| e.lines().map(|c| c.parse::<u32>().unwrap()).sum::<u32>())
            .max()
            .unwrap(),
    );
}
 */