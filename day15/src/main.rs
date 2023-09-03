use std::collections::HashSet;

fn main() {
    let data = include_str!("../input.txt");
    // part1(data);
    println!("=====");
    part2(data);
}

fn sensor_beason_pairs(data: &str) -> Vec<(i64, i64, i64, i64)> {
    let lex_xy = |xy_str: &str| {
        // Example xy_str: "x=3797530, y=3451192"
        let (x_str, y_str) = xy_str.split_once(", ").unwrap();
        (x_str[2..].parse::<i64>().unwrap(), y_str[2..].parse::<i64>().unwrap())
    };
    data.lines()
        .map(|line| {
            let (sensor_str, beacon_str) = line.split_once(": closest beacon is at ").unwrap();
            let sensor_str = sensor_str.split_once("Sensor at ").unwrap().1;
            let (sx, sy) = lex_xy(sensor_str);
            let (bx, by) = lex_xy(beacon_str);
            (sx, sy, bx, by)
        })
        .collect()
}


fn part1(data: &str) {
    const Y_COORD_TO_TEST: i64 = 2000000;
    let sb_pair_coords = sensor_beason_pairs(data);

    let (mut x_min, mut x_max) = (i64::MAX, i64::MIN);
    let mut x_of_beacons_in_row: HashSet<i64> = HashSet::new();
    let mut blocked_segments: Vec<(i64, i64)> = Vec::new();
    for (sx, sy, bx, by) in sb_pair_coords {
        let dist = (sx.abs_diff(bx) + sy.abs_diff(by)) as i64;
        let remaining_dist = dist - sy.abs_diff(Y_COORD_TO_TEST) as i64;
        if remaining_dist >= 0 {
            let (start, end_inclusive) = (sx - remaining_dist, sx + remaining_dist);
            x_min = x_min.min(start);
            x_max = x_max.max(end_inclusive);
            blocked_segments.push((start, end_inclusive));
        }

        if by == Y_COORD_TO_TEST {
            x_of_beacons_in_row.insert(bx);
        }
    }
    let blocked_segments = blocked_segments;
    let x_of_beacons_in_row = x_of_beacons_in_row;
    println!("({}..={})", x_min, x_max);

    let position_count = (x_min..=x_max)
        .filter(|x| {
            if x_of_beacons_in_row.contains(x) {
                false
            } else {
                blocked_segments.iter()
                    .any(|(start, end_inclusive)| {
                        start <= x && x <= end_inclusive
                    })
            }
        })
        .count();

    println!("{}", position_count);
}


const QUESTION_CONST: usize = 4000000;
fn part2(data: &str) {
    let sb_pair_coords = sensor_beason_pairs(data);
    for y_test in 0..=QUESTION_CONST {
        let mut excluded_x = [true; QUESTION_CONST+1];
        for (sx, sy, bx, by) in sb_pair_coords.iter() {
            let dist = (sx.abs_diff(*bx) + sy.abs_diff(*by)) as i64;
            let remaining_dist = dist - sy.abs_diff(y_test as i64) as i64;
            if remaining_dist >= 0 {
                let start = 0i64.max(sx - remaining_dist) as usize;
                let end_inclusive = (QUESTION_CONST as i64).min(sx + remaining_dist) as usize;
                excluded_x[start..=end_inclusive].fill(true);
            }
        }
        let candidate: Vec<usize> = excluded_x.iter()
            .enumerate()
            .filter_map(|(i, excluded)| {
                if *excluded { None } else { Some(i) }
            }).collect();
        if candidate.len() != 0 {
            println!("x = {}, y = {}, freq={}", candidate[0], y_test, candidate[0] * QUESTION_CONST);
        }
    }
}
