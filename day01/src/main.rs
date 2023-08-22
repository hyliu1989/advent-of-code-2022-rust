fn insert(a: &mut [u32; 3], val: &u32) {
    if *val <= a[2] {
        return;
    }
    a[2] = *val;
    for i in (0..2).rev() {
        if a[i+1] > a[i] {
            a.swap(i, i+1)
        }
    }
}

fn main() {
    let data = include_str!("../input.txt");
    let mut max = 0u32;
    let mut top_maxes = [0u32, 0u32, 0u32];
    for elf_str in data.split("\n\n") {
        let mut current_ration = 0u32;
        for ration_str in elf_str.lines() {
            let ration = ration_str.parse::<u32>().unwrap();
            current_ration += ration;
        }
        insert(&mut top_maxes, &current_ration);
        if max < current_ration {
            max = current_ration;
        }
    }
    println!("{}", max);
    println!("{} {} {}", top_maxes[0], top_maxes[1], top_maxes[2]);
    println!("{}", top_maxes.into_iter().sum::<u32>());
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