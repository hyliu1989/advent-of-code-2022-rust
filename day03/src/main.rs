fn main() {
    part1();
    println!("=============");
    part2();
}

fn get_priority(c: char) -> u8 {
    let id = (c as u8) - ('A' as u8);
    if id > 26 {
        (c as u8) - ('a' as u8)
    } else {
        id + 26
    }
}

fn part1() {
    let data = include_str!("../input.txt");
    let mut sum_priority_of_repeated = 0u32;
    for l in data.lines() {
        let n = l.len() / 2;
        let mut record = [vec![0u32; 52], vec![0u32; 52]];
        for (i, c) in l.char_indices() {
            record[i / n][get_priority(c) as usize] += 1;
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

fn part2() {
    let data = include_str!("../input.txt");
    let sum_priority = data.lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|line_batch| {
            let mut record: [Vec<u32>; 3] = [vec![0u32; 52], vec![0u32; 52], vec![0u32; 52]];
            for i in 0..3 {
                for c in line_batch[i].chars() {
                    record[i][get_priority(c) as usize] += 1;
                }
            }
            for i in 0..52 {
                if record[0][i] != 0 && record[1][i] != 0 && record[2][i] != 0 {
                    return (i as u32) + 1;
                }
            }
            return 0;
        })
        .sum::<u32>();

    println!("{}", sum_priority);
}
