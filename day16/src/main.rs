use std::collections::{HashMap, VecDeque, HashSet};
use itertools::Itertools;

const DEBUG: bool = false;

fn main() {
    /*
    The overview of the algorithm: Figure out only the relavent volves that has nonzero pressure release ability.
    And then deal with the permutation of the relavent volves by meeting-in-the-middle method.
    The memoising keys are the selected volves in the first half (7 volves) and the last volve of the 7.
    The memoising values are the remaining turns/moves and terminal pressure flows of the opened volves.
    */
    let data = include_str!("../input.txt");
    let (node_neighbors, 
         relavent_node_ids, 
         relavent_node_rate) = parse_neighbors_and_relavent_volves(data);

    // Build shorted distances from a relavent node to another. A relavent node is
    // one that has non zero pressure flow or is the start node.
    let num_rel_nodes = relavent_node_ids.len();
    let mut mutual_dist = vec![vec![0; num_rel_nodes]; num_rel_nodes];
    for (src_node, src_id) in relavent_node_ids.iter() {
        let mut visited: HashSet<&str> = HashSet::new();
        let mut queue: VecDeque<(&str, i32)> = VecDeque::new();
        queue.push_back((*src_node, 0));
        visited.insert(*src_node);
        while let Some((curr, curr_dist)) = queue.pop_front() {
            // Add the currently traversed node to the shortest distance record.
            if let Some(curr_id) = relavent_node_ids.get(curr) {
                mutual_dist[*src_id][*curr_id] = curr_dist;
            }
            // Add unvisited neighbors of curr to the queue.
            for des in node_neighbors.get(curr).unwrap() {
                if visited.contains(*des) {
                    continue;
                }
                visited.insert(*des);
                queue.push_back((*des, curr_dist+1));
            }
        }
    }
    let mutual_dist = mutual_dist;
    if DEBUG {
        println!("The distance matrix:");
        for row in mutual_dist.iter() {
            println!("{:2?}", row);
        }
        println!("The pressure rates: {:?}", relavent_node_rate);

        let one_result = open_volves(
            &relavent_node_rate, 
            &mutual_dist, 
            // &([0, 3, 6, 1, 2, 4, 5].into()), 
            &([0, 3, 1, 6, 5, 4, 2].into()), 
            30, 
            true,
        );
        println!("one result: {} {}", one_result.0, one_result.1);
    }

    // find_best_volve_opening::<30>(&relavent_node_rate, &mutual_dist);
    find_best_volve_opening_2(&relavent_node_rate, &mutual_dist);
    println!("============");
    const TURN_LIMIT_2: i32 = 26;
    let mut max_score = 0i32;
    for num_mine in 1..7 {
        for combination_selection in (1usize..16).into_iter().combinations(num_mine) {
            let me_subset = make_selection_volve_mask(&combination_selection) | 0x0001;
            let element_subset = (!me_subset) | 0x0001;
            let score = 
                find_next_volve_subset(
                    0u16,
                    0,
                    TURN_LIMIT_2,
                    &relavent_node_rate,
                    &mutual_dist,
                    &mut HashMap::new(),
                    me_subset,
                ) + find_next_volve_subset(
                    0u16,
                    0,
                    TURN_LIMIT_2,
                    &relavent_node_rate,
                    &mutual_dist,
                    &mut HashMap::new(),
                    element_subset,
                );
            max_score = max_score.max(score);
        }
    }
    println!("{}", max_score)
}


fn parse_neighbors_and_relavent_volves(data: &str) -> (HashMap<&str, Vec<&str>>, HashMap<&str, usize>, Vec<i32>) {
    // Build graph for node connections and their distances in terms of moving time
    let mut node_neighbors: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut relavent_node_ids: HashMap<&str, usize> = HashMap::new();  // Assign numeric ids to relavent nodes.
    let mut relavent_node_rate: Vec<i32> = Vec::new();

    let mut id_used = 0usize;
    relavent_node_ids.insert("AA", id_used);
    relavent_node_rate.push(0);
    id_used += 1;
    for l in data.lines() {
        // Example: "Valve RT has flow rate=0; tunnels lead to valves EN, LZ"
        let curr = &l[6..8];
        let curr_neighbor_str = l.split_once("to valves ");
        let curr_neighbor_str = match curr_neighbor_str {
            Some(_) => { curr_neighbor_str },
            None => { l.split_once("to valve ")},
        }.unwrap().1;
        let curr_neighbor: Vec<&str> = curr_neighbor_str
            .split(", ")
            .map(|s| {s})
            .collect();
        let flow_rate = l
            .split_once("flow rate=").unwrap().1
            .split_once(";").unwrap().0
            .parse::<i32>().unwrap();
        if flow_rate != 0 {
            let k = curr.clone();
            if !relavent_node_ids.contains_key(&k) {
                relavent_node_ids.insert(k, id_used);
                relavent_node_rate.push(flow_rate);
                id_used += 1;
            } else {
                relavent_node_rate[*relavent_node_ids.get(&k).unwrap()] = flow_rate;
            }
        }
        node_neighbors.insert(curr, curr_neighbor);
    }
    (node_neighbors, relavent_node_ids, relavent_node_rate)
}


fn open_volves(
    volve_rate: &Vec<i32>, volve_distances: &Vec<Vec<i32>>, volves_path: &Vec<usize>, turn_limit: i32, go_until_turn_limit: bool
) -> (i32, i32) {
    let mut prev_volve: usize = volves_path[0];
    let mut num_turns: i32 = 0;
    let mut score: i32 = 0;
    let mut curr_pressure_release_rate = 0;
    for volve in volves_path {  // including volves_path[0] to properly open the volve [0].
        let volve = *volve;
        let mut num_turns_local = volve_distances[prev_volve][volve];
        if volve_rate[volve] != 0 {
            num_turns_local += 1;
        }
        num_turns += num_turns_local;
        if num_turns >= turn_limit {
            let exceed = num_turns - turn_limit;
            num_turns -= exceed;
            num_turns_local -= exceed;
        }
        score += curr_pressure_release_rate * num_turns_local;
        curr_pressure_release_rate += volve_rate[volve];
        if num_turns == turn_limit {
            break;
        }
        prev_volve = volve;
    }
    if !go_until_turn_limit {
        (num_turns, score)
    } else {
        (turn_limit, score + (turn_limit-num_turns) * curr_pressure_release_rate)
    }
}


fn find_best_volve_opening<const TURN_LIMIT: i32>(volve_rate: &Vec<i32>, volve_distances: &Vec<Vec<i32>>) {
    const N_MEET_AT_THE_MIDDLE: usize = 3;
    let num_relavent_volves = volve_rate.len();
    assert!(volve_rate[0] == 0);  // Assume the start point does not have any flow value in its volve.
    // The DP cache maps (mask of selected volves, last node) to a vector of (remaining counts, current score).
    // The reason to use a vector is that different remaining turns are different tracks, unless there is a superior
    // track that has both more counts and more score than another.
    let mut max_release_pressure = 0;

    for comb_sel in (1..num_relavent_volves).into_iter().combinations(N_MEET_AT_THE_MIDDLE) {
        let mut dp: HashMap<usize, Vec<(i32, i32)>> = HashMap::new();
        let comb_sel = vec![0, 3, 1, 6]; // TODO: remove me
        for permutation_seq in [vec![0, 3, 1, 6]] {  // TODO: revert to comb_sel.iter().permutations()
            let mut volves_path = vec![0];
            volves_path.extend(permutation_seq.into_iter());
            let (num_turns, score) = open_volves(&volve_rate, &volve_distances, &volves_path, TURN_LIMIT, false);
            if num_turns == TURN_LIMIT {
                max_release_pressure = max_release_pressure.max(score);
                continue;
            }
            // Update the memoized cache.
            let k = &(volves_path[volves_path.len()-1]);
            let mut recorded_results = dp.remove(k).unwrap_or(Vec::new());
            let mut new_recorded: Vec<(i32, i32)> = Vec::new();
            let mut to_use = true;
            let mut used_to_cover_another = false;
            for (rec_turns, rec_score) in recorded_results.drain(..) {
                if num_turns >= rec_turns && score <= rec_score {
                    to_use = false;
                    assert!(!used_to_cover_another);
                    new_recorded.push((rec_turns, rec_score));
                } else if rec_turns >= num_turns && rec_score <= score {
                    // Dropping the old record because that can be covered by the current turns and scores.
                    used_to_cover_another = true;  // Guarding the usage of the current score.
                    assert!(to_use);
                } else {
                    new_recorded.push((rec_turns, rec_score));
                }
            }
            if to_use {
                new_recorded.push((num_turns, score));
            }
            dp.insert(*k, new_recorded);
            println!("{:?} {}", volves_path, score);
        }

        let second_half_combination = (1..num_relavent_volves)
            .filter_map(|volve| { if comb_sel.contains(&volve) {None} else { Some(volve) } })
            .collect_vec();
        let terminal_pressure_release_rate_first_half: i32 = comb_sel.iter()
            .map(|volve| {volve_rate[*volve]})
            .sum();
        let mut second_half_dp: HashMap<i32, i32> = HashMap::new();  // maps remaining turns to second-half-only score.
        for permutation_seq in second_half_combination.iter().permutations(second_half_combination.len()) {
            let permutation_seq = permutation_seq.into_iter().map(|s| *s).collect_vec();
            // connecting the first and the second half.
            for (first_half_last_volve, first_half_turns_and_score) in dp.iter() {
                for (first_half_turns, first_half_score) in first_half_turns_and_score {
                    let mut total_score = *first_half_score
                        + terminal_pressure_release_rate_first_half
                        * (TURN_LIMIT - first_half_turns);
                    let mut num_remaining_turns = TURN_LIMIT - first_half_turns;
                    num_remaining_turns -= volve_distances[*first_half_last_volve][permutation_seq[0]];
                    if num_remaining_turns > 0 {
                        total_score += match second_half_dp.get(&num_remaining_turns) {
                            Some(score) => { *score },
                            None => {
                                let (_, score) = open_volves(
                                    &volve_rate, 
                                    &volve_distances,
                                    &permutation_seq,  // FIXME: we calculate only one permutation and then we call it the max and cache it! This is wrong.
                                    num_remaining_turns,
                                    true,
                                );
                                second_half_dp.insert(num_remaining_turns, score);
                                score
                            },
                        };
                    }
                    println!("{}, {}, {:?}, {}, {:?}", total_score, first_half_score, comb_sel, first_half_last_volve, permutation_seq);
                    max_release_pressure = max_release_pressure.max(total_score);
                }
            }
        }
        break;  // TODO: remove me
    }
    println!("max_release_pressure = {}", max_release_pressure);
}


fn find_best_volve_opening_2(volve_rate: &Vec<i32>, volve_distances: &Vec<Vec<i32>>) {
    const TURN_LIMIT: i32 = 30;
    assert!(volve_rate.len() <= 16);
    let mut cache = HashMap::new();
    println!(
        "{} (cache size {})",
        find_next_volve_subset(0u16, 0, TURN_LIMIT, volve_rate, volve_distances, &mut cache, 0xffff),
        cache.len(),

    );
}


fn find_next_volve_subset(
    mut opened_volve: u16,
    next_volve: usize,
    mut remaining_turns: i32,
    volve_rate: &Vec<i32>,
    volve_distances: &Vec<Vec<i32>>,
    cache: &mut HashMap<(u16, usize, i32), i32>,
    subset: u16,
) -> i32 {
    let k = &(opened_volve, next_volve, remaining_turns);
    if let Some(score) = cache.get(k) {
        return *score;
    }
    let curr_volve = next_volve;
    if volve_rate[curr_volve] != 0 {
        remaining_turns -= 1;
    }
    let total_pressure_by_curr = volve_rate[curr_volve] * remaining_turns;
    opened_volve |= 1u16 << curr_volve;

    let mut max_pressure = 0i32;
    for next_volve in 0..volve_rate.len() {
        let next_volve_mask = 1u16 << next_volve;
        if (subset & next_volve_mask) == 0 {
            // Not in subset. Abort.
            continue;
        }

        if (opened_volve & next_volve_mask) != 0 {
            // Already opened. Abort.
            continue;
        }
        let local_remaining_turns = remaining_turns - volve_distances[curr_volve][next_volve];
        if local_remaining_turns <= 0 {
            continue;
        }
        max_pressure = max_pressure.max(
            find_next_volve_subset(
                opened_volve, next_volve, local_remaining_turns, volve_rate, volve_distances, cache, subset
            )
        );
    }
    cache.insert(*k, total_pressure_by_curr + max_pressure);
    total_pressure_by_curr + max_pressure
}

fn make_selection_volve_mask(volves: &Vec<usize>) -> u16 {
    volves.into_iter().map(|v| {1u16 << v}).sum()
}
