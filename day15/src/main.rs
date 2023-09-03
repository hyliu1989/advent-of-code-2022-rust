use std::collections::{HashSet, VecDeque};

fn main() {
    let data = include_str!("../input.txt");
    part1(data);
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

    let mut blocked_segments:VecDeque<(i64, i64)> = sb_pair_coords.iter()
        .filter_map(|(sx, sy, bx, by)| {
            let dist = (sx.abs_diff(*bx) + sy.abs_diff(*by)) as i64;
            let remaining_dist = dist - sy.abs_diff(Y_COORD_TO_TEST) as i64;
            if remaining_dist >= 0 {
                let start = sx - remaining_dist;
                let end_inclusive = sx + remaining_dist;
                Some((start, end_inclusive))
            } else {
                None
            }
        })
        .collect();

    let x_of_beacons_in_row: HashSet<i64> = sb_pair_coords.iter()
        .filter_map(|(_, _, bx, by)| {if *by == Y_COORD_TO_TEST { Some(*bx) } else { None }})
        .collect();
    
    for bx in x_of_beacons_in_row.iter() {
        blocked_segments.push_back((*bx, *bx));
    }

    let non_overlapping_segments = merge_segments(blocked_segments);

    let num_blocked_positions: i64 = non_overlapping_segments.into_iter()
        .map(|(start, end_inclusive)| { end_inclusive - start + 1 })
        .sum::<i64>()
        - (x_of_beacons_in_row.len() as i64);

    println!("{}", num_blocked_positions);
}


fn merge_segment_pair(segment1: (i64, i64), segment2: (i64, i64)) -> Option<(i64, i64)> {
    let (start1, end1) = segment1;
    let (start2, end2) = segment2;

    // Check if the segments overlap
    if end1 >= start2 && end2 >= start1 {
        // Calculate the merged segment
        let merged_start = start1.min(start2);
        let merged_end = end1.max(end2);
        Some((merged_start, merged_end))
    } else {
        None
    }
}

/* Returns a Vec of nonoverlapping segments. */
fn merge_segments(mut segments: VecDeque<(i64, i64)>) -> Vec<(i64, i64)> {
    let mut ret: Vec<(i64, i64)> = Vec::new();
    while segments.len() != 0 {
        let seg1 = segments.pop_front().unwrap();
        let num_operation = segments.len();
        let mut merged = false;
        for _ in 0..num_operation {
            let seg2 = segments.pop_front().unwrap();
            match merge_segment_pair(seg1, seg2) {
                Some(new_seg) => {
                    merged = true;
                    segments.push_back(new_seg);
                },
                None => {
                    segments.push_back(seg2);
                },
            }
        }
        if !merged {
            ret.push(seg1);
        }
    }
    ret
}


fn part2(data: &str) {
    const QUESTION_CONST: usize = 4000000;
    let sb_pair_coords = sensor_beason_pairs(data);
    for y_test in (0..=QUESTION_CONST).rev() {
        let blocked_segments:VecDeque<(i64, i64)> = sb_pair_coords.iter()
            .filter_map(|(sx, sy, bx, by)| {
                let dist = (sx.abs_diff(*bx) + sy.abs_diff(*by)) as i64;
                let remaining_dist = dist - sy.abs_diff(y_test as i64) as i64;
                if remaining_dist >= 0 {
                    let start = 0i64.max(sx - remaining_dist);
                    let end_inclusive = (QUESTION_CONST as i64).min(sx + remaining_dist);
                    Some((start, end_inclusive))
                } else {
                    None
                }
            })
            .collect();
        let non_overlapping_segments = merge_segments(blocked_segments);
        if non_overlapping_segments.len() > 1 {
            let mut x_coord = 0i64;
            println!("Segments: ");
            for seg in non_overlapping_segments.iter() {
                println!("- ({}, {})", seg.0, seg.1);
                if seg.0 == 0 {
                    x_coord = seg.1 + 1;
                }
            }
            let y_coord = y_test;
            println!("x = {}, y = {}, freq = {}", x_coord, y_coord, (x_coord as usize) * QUESTION_CONST + y_coord);
            break;
        }
    }
}
