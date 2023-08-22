fn main() {
    part1();
    println!("===================");
    part2();
}

fn part1() {
    let data = include_str!("../input.txt");
    let mut score = 0u32;
    for l in data.lines() {
        let char_array: Vec<char> = l.chars().collect();
        let oppo = char_array[0];
        let mine = char_array[2];
        let mut winning: char = 'Y';
        let mut losing: char = 'X';
        match oppo {
            'A' => { winning = 'Y'; losing = 'Z'; },
            'B' => { winning = 'Z'; losing = 'X'; },
            'C' => { winning = 'X'; losing = 'Y'; },
            _ => {},
        }
        match mine {
            'X' => score += 1,
            'Y' => score += 2,
            'Z' => score += 3,
            _ => {},
        }
        if mine == winning {
            score += 6;
        } else if mine == losing {
            //
        } else {
            score += 3;
        }
    }
    println!("{}", score);

    // println!(
    //     "{}",
    //     include_bytes!("../input.txt")
    //         .split(|b| *b == b'\n')
    //         .map(|l| ((l[0] - b'A') as i16, (l[2] - b'X') as i16,))
    //         .map(|(a, b)| 1 + b + 3 * (1 + b - a).rem_euclid(3))
    //         .sum::<i16>(),
    // );
}

fn part2() {
    let data = include_str!("../input.txt");
    let mut score = 0u32;
    for l in data.lines() {
        let char_array: Vec<char> = l.chars().collect();
        let oppo = (char_array[0] as i8) - ('A' as i8);
        let mut mine: i8 = 0;
        let expected_outcome = char_array[2];
        match expected_outcome {
            'X' => {
                // To lose
                mine = oppo - 1;
            },
            'Y' => {
                // To draw
                score += 3;
                mine = oppo;
            },
            'Z' => {
                // To win
                score += 6;
                mine = oppo + 1;
            },
            _ => {},
        }
        match mine.rem_euclid(3) {
            0 => score += 1,
            1 => score += 2,
            2 => score += 3,
            _ => {},
        }
    }
    println!("{}", score);

    // println!(
    //     "{}",
    //     include_bytes!("../input.txt")
    //         .split(|b| *b == b'\n')
    //         .map(|l| ((l[0] - b'A') as i16, (l[2] - b'X') as i16,))
    //         .map(|(a, b)| 1 + b * 3 + (2 + a + b) % 3)
    //         .sum::<i16>(),
    // );
}
