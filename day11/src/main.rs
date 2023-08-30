
type WorryLevel = u64;

struct Monkey {
    holding: Vec<WorryLevel>,
    insp_op: Box<dyn Fn(WorryLevel) -> WorryLevel>,
    test_op: Box<dyn Fn(WorryLevel) -> usize>,
    test_divisor: u32,
    count: u32,
}

fn build_monkeys(data: &str) -> Vec<Monkey>{
    data.split("\n\n")
        .map(|monkey_input| {
            let lines: Vec<&str> = monkey_input.lines().collect();
            let holding_gen = lines[1]
                .split_once("  Starting items: ").unwrap().1.split(", ")
                .map(|num_str| num_str.parse::<WorryLevel>().unwrap());
            let insp_op: Box<dyn Fn(WorryLevel) -> WorryLevel> = {
                let terms: Vec<&str> = lines[2].split_once("new = ").unwrap().1.split(" ").collect();
                assert!(terms[0] == "old");
                match terms[2] {
                    "old" => { Box::new(move |old: WorryLevel| {old * old}) },
                    n_str => {
                        match (terms[1], n_str.parse::<WorryLevel>().unwrap()) {
                            ("*", n) => { Box::new(move |old: WorryLevel| { old * n }) },
                            ("+", n) => { Box::new(move |old: WorryLevel| { old + n }) },
                            _ => unreachable!(),
                        }
                    },
                }
            };
            let divisor: u32;
            let test_op: Box<dyn Fn(WorryLevel) -> usize> = {
                let parse = |line: &str, delim: &str| {
                    line.split_once(delim).unwrap().1.parse::<WorryLevel>().unwrap()
                };
                let divisor_loc = parse(lines[3], "divisible by ");
                divisor = divisor_loc as u32;
                let true_ret = parse(lines[4], "throw to monkey ") as usize;
                let false_ret = parse(lines[5], "throw to monkey ") as usize;
                Box::new(
                    move |worry_level| { if worry_level % divisor_loc == 0 { true_ret } else {false_ret} }
                )
            };
            Monkey { holding: holding_gen.collect::<Vec<WorryLevel>>(), insp_op, test_op, test_divisor: divisor, count: 0 }
        })
        .collect()
}

fn part1(data: &str) {
    let mut monkeys = build_monkeys(data);

    let mut temp_bags = vec![vec![0; 0]; monkeys.len()];
    for _ in 0..20 {
        for (i, m) in monkeys.iter_mut().enumerate() {
            m.holding.append(&mut temp_bags[i]);
            for old in m.holding.drain(..) {
                let worry_level = (m.insp_op)(old) / 3;
                let dest_monkey = (m.test_op)(worry_level);
                temp_bags[dest_monkey].push(worry_level);
                m.count += 1;
            }
        }
    }

    let mut counts: Vec<u32> = monkeys.iter().map(|m| m.count).collect();
    counts.sort();
    println!("{}", counts[counts.len()-1] * counts[counts.len()-2]);
}

fn part2(data: &str) {
    let mut monkeys = build_monkeys(data);

    // Use modular number to keep the worry level under control.
    // This would not affect the division test.
    let worry_level_reduction_div: u32 = monkeys.iter().map(|m| m.test_divisor).product();
    let worry_level_reduction_div = worry_level_reduction_div as WorryLevel;

    let mut temp_bags = vec![vec![0; 0]; monkeys.len()];
    for _ in 0..10000 {
        for (i, m) in monkeys.iter_mut().enumerate() {
            m.holding.append(&mut temp_bags[i]);
            for old in m.holding.drain(..) {
                let worry_level = (m.insp_op)(old) % worry_level_reduction_div;
                let dest_monkey = (m.test_op)(worry_level);
                temp_bags[dest_monkey].push(worry_level);
                m.count += 1;
            }
        }
    }

    let mut counts: Vec<u32> = monkeys.iter().map(|m| m.count).collect();
    counts.sort();
    println!("{}", (counts[counts.len()-1] as u64) * (counts[counts.len()-2] as u64));
}

fn main() {
    let data = include_str!("../input.txt");
    part1(data);
    println!("=============");
    part2(data);
}
