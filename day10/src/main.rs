fn main() {
    let data = include_str!("../input.txt");
    part1(data);
    println!("=================");
}

fn part1(data: &str) {
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
    println!(
        "{}",
        [20, 60, 100, 140, 180, 220]
            .map(|cycle_serial: i32| {record[(cycle_serial as usize)-1] * cycle_serial})
            .into_iter()
            .sum::<i32>(),
    );

}
