fn main() {
    let data = include_str!("../input.txt");
    let mut record: Vec<i32> = Vec::new();  // element:  value of X reg
    let mut x: i32 = 1;
    for line in data.lines() {
        match line {
            "noop" => { record.push(x); },
            add_inst => {
                record.push(x);
                record.push(x);
                let val = add_inst.split_once(' ').unwrap().1.parse::<i32>().unwrap();
                x += val;
            }
        }
    }
    part1(&record);
    println!("=================");
    part2(&record);
}

fn part1(record: &Vec<i32>) {
    println!(
        "{}",
        [20, 60, 100, 140, 180, 220]
            .map(|cycle_serial: i32| {record[(cycle_serial as usize)-1] * cycle_serial})
            .into_iter()
            .sum::<i32>(),
    );
}

fn part2(record: &Vec<i32>) {
    let mut ret = [' '; 240];
    for idx in 0..240usize {
        let i_crt_pos = (idx as i32) % 40;
        if record[idx].abs_diff(i_crt_pos) <= 1 {
            ret[idx] = '#';
        }
    }
    for crt_line in ret.chunks_exact(40) {
        let string: String = crt_line.iter().collect();
        println!("{}", string);
    }
    
}
