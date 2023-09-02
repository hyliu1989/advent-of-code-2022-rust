use std::cmp::Ordering;

fn main() {
    let data = include_str!("../input.txt");
    part1(data);
    println!("=============");
}

#[derive(PartialEq, Eq, Debug)]
enum PacketData {
    Num(i32),
    List(Vec<PacketData>),
}
impl Ord for PacketData {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            // Compare the numbers.
            (PacketData::Num(a), PacketData::Num(b)) => {
                a.cmp(b)
            },
            // Compare two lists.
            (PacketData::List(a), PacketData::List(b)) => {
                a.iter().cmp(b)
            },
            // Augment the number to be a list and then compare two lists.
            (PacketData::Num(a), PacketData::List(_)) => {
                let augmented_list = PacketData::List(vec![PacketData::Num(*a)]);
                augmented_list.cmp(other)
            },
            // Compare a number and a list using the reverse.
            (PacketData::List(_), PacketData::Num(_)) => {
                other.cmp(self).reverse()
            },
        }
    }
}

impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_packet_line(line: &[u8]) -> PacketData {
    assert!(line[0] == b'[' && line[line.len()-1] == b']');

    let mut trace: Vec<Vec<PacketData>> = vec![Vec::new()];
    
    let mut i = 1usize;
    while i < line.len() - 1 {
        match line[i] {
            b'[' => {
                trace.push(Vec::new());
                i += 1;
            },
            b']' => {
                let finished_list = trace.pop().unwrap();
                trace.last_mut().unwrap().push(PacketData::List(finished_list));
                i += 1;
                if line[i] == b',' {
                    i += 1;
                }
            },
            b'0'..=b'9' => {
                let mut num_string_len = 1;
                loop {
                    match line[i + num_string_len] {
                        b'0'..=b'9' => { num_string_len += 1; },
                        _ => { break; }
                    }
                }
                let num_string_len = num_string_len;
                let mut parsed_num: i32 = 0;
                line[i..i+num_string_len].iter().for_each(|digit| {
                    parsed_num *= 10;
                    parsed_num += (*digit as i32) - (b'0' as i32);
                });
                trace.last_mut().unwrap().push(PacketData::Num(parsed_num));

                if line[i+num_string_len] == b',' {
                    i = i+num_string_len + 1;
                } else {
                    i = i+num_string_len
                }
            },
            _f => { println!("unknown char {}", _f); i += 1;},
        }
    }
    let ret = PacketData::List(trace.pop().unwrap());
    assert!(trace.len() == 0);
    return ret;
}

fn part1(data: &str) {
    let count = data.split("\n\n")
        .map(|two_lines|{
            let mut two_line_iter = two_lines.lines();
            (
                parse_packet_line(two_line_iter.next().unwrap().as_bytes()),
                parse_packet_line(two_line_iter.next().unwrap().as_bytes()),
            )
        })
        .enumerate()
        .filter(|(_, (left, right))| { left < right })
        .map(|(i, _)| { i + 1 })
        .sum::<usize>();
    println!("{}", count);
}
