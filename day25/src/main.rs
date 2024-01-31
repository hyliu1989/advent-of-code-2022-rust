use std::collections::VecDeque;

fn five_unit_integer(regular_int: usize) -> Vec<i8> {
    let mut five_unit_int = Vec::new();
    let mut int = regular_int;
    while int > 0 {
        five_unit_int.push((int % 5) as i8);
        int /= 5;
    }
    five_unit_int
}

fn reg_to_snafu(regular_int: usize) -> Vec<i8> {
    let five_unit_int: Vec<i8> = five_unit_integer(regular_int);
    let mut snafu_int: Vec<i8> = Vec::new();
    snafu_int.reserve(five_unit_int.len() + 1);
    let mut accum = 0i8;
    for i in five_unit_int.iter() {
        accum += i;
        match accum {
            0 => {snafu_int.push(0); accum = 0;},
            1 => {snafu_int.push(1); accum = 0;},
            2 => {snafu_int.push(2); accum = 0;},
            3 => {snafu_int.push(-2); accum = 1;},
            4 => {snafu_int.push(-1); accum = 1;},
            5 => {snafu_int.push(0); accum = 1;},
            _ => panic!("accum is {}", accum),
        }
    }
    match accum {
        0 => {},
        1 => {snafu_int.push(1);},
        _ => panic!("tailing accum is {}", accum),
    }
    snafu_int
}

fn snafu_to_reg<'a, T>(snafu_int: T) -> usize
    where T: IntoIterator<Item = &'a i8>
{
    let mut regular_int = 0isize;
    let mut base = 1isize;
    for i in snafu_int.into_iter() {
        regular_int += ((*i) as isize) * base;
        base *= 5;
    }
    regular_int as usize
}


fn main() {
    println!("Hello, world!");
    let data = include_bytes!("../input.txt");
    let list_of_snafu: Vec<VecDeque<i8>> = 
        data.split(|&x| x == b'\n')
        .filter(|x| x.len() != 0)
        .map(|x| {
            let mut snafu_int = VecDeque::new();
            for c in x.iter() {
                match c {
                    b'=' => snafu_int.push_front(-2),
                    b'-' => snafu_int.push_front(-1),
                    b'0' => snafu_int.push_front(0),
                    b'1' => snafu_int.push_front(1),
                    b'2' => snafu_int.push_front(2),
                    _ => panic!("unexpected char {}", c),
                }
            }
            snafu_int
        })
        .collect();
    let sum: usize = list_of_snafu.iter().map(|x| snafu_to_reg(x)).sum();
    let sum_snafu_str: String = reg_to_snafu(sum).iter()
        .rev()
        .map(|x| match x { -2 => '=', -1 => '-', 0 => '0', 1 => '1', 2 => '2', _ => panic!("unexpected char {}", x) })
        .collect();
    println!("sum_snafu_str: '{}'", sum_snafu_str);
}
