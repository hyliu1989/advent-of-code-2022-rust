fn main() {
    part1();
}


fn part1() {
    let data = include_str!("../input.txt");
    let mut sum_priority_of_repeated = 0u32;
    for l in data.lines() {
        let n = l.len() / 2;
        let mut record = [vec![0u32; 52], vec![0u32; 52]];
        for (i, c) in l.char_indices() {
            let mut id = (c as u8) - ('A' as u8);
            id = if id > 26 { (c as u8) - ('a' as u8) } else { id + 26 };
            record[i / n][id as usize] += 1;
        }
        for i in 0..52 {
            if record[0][i] != 0 && record[1][i] != 0 {
                sum_priority_of_repeated += (i as u32) + 1;
                break;
            }
        }
    }
    println!("{}", sum_priority_of_repeated);
}
