fn main() {
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
