fn main() {
    let data = include_str!("../input.txt");
    let blue_prints = parse_input(data);
    part1(&blue_prints);
    println!("Hello, world!");
    println!("{:#?}", blue_prints);
}

#[derive(Debug)]
struct BluePrint {
    id: u8,
    ore_bot_cost: (u8,),
    clay_bot_cost: (u8,),
    obsidian_cost: (u8, u8),
    geode_bot_cost: (u8, u8),
    ore_consume_max: u8,
}

#[derive(Clone)]
struct State {
    time: u16,
    ore: u16,
    clay: u16,
    obsidian: u16,
    geode: u16,
    ore_bot: u16,
    clay_bot: u16,
    obsidian_bot: u16,
    geode_bot: u16,
}

enum RobotType { Ore, Clay, Obsidian, Geode }

fn parse_input(data: &str) -> Vec<BluePrint> {
    // Blueprint 1: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 20 clay. Each geode robot costs 4 ore and 7 obsidian.
    data.lines()
        .map(|l| {
            let (id_str, blueprint_str) = l.split_once(":").unwrap();
            let id = id_str.split_once(" ").unwrap().1.parse::<u8>().unwrap();
            let mut iter = blueprint_str.split(" Each ").skip(1);
            let ore_str = iter.next().unwrap();
            assert!(&ore_str[..3] == "ore");
            let ore_robot = (ore_str.split(" ").into_iter().nth(3).unwrap().parse::<u8>().unwrap(),);
            
            let clay_str = iter.next().unwrap();
            assert!(&clay_str[..4] == "clay");
            let clay_robot = (clay_str.split(" ").into_iter().nth(3).unwrap().parse::<u8>().unwrap(),);

            let obsi_str = iter.next().unwrap();
            assert!(&obsi_str[..8] == "obsidian");
            let obsidian = (
                obsi_str.split(" ").into_iter().nth(3).unwrap().parse::<u8>().unwrap(),
                obsi_str.split(" ").into_iter().nth(6).unwrap().parse::<u8>().unwrap(),
            );

            let geod_str = iter.next().unwrap();
            assert!(&geod_str[..5] == "geode");
            let geode = (
                geod_str.split(" ").into_iter().nth(3).unwrap().parse::<u8>().unwrap(),
                geod_str.split(" ").into_iter().nth(6).unwrap().parse::<u8>().unwrap(),
            );

            BluePrint { id, ore_bot_cost: ore_robot, clay_bot_cost: clay_robot, obsidian_cost: obsidian, geode_bot_cost: geode, 
                ore_consume_max: ore_robot.0.max(clay_robot.0).max(obsidian.0).max(geode.0),
            }
        })
        .collect()
}

fn part1(blueprints: &Vec<BluePrint>) {
    let mut total_quality_level: u32 = 0;
    for bp in blueprints {
        let mut geode_max: u16 = search_next::<24>(
            bp,
            State { time: 0, ore: 0, clay: 0, obsidian: 0, geode: 0, ore_bot: 1, clay_bot: 0, obsidian_bot: 0, geode_bot: 0 },
            RobotType::Ore,
        );
        geode_max = geode_max.max(search_next::<24>(
            bp,
            State { time: 0, ore: 0, clay: 0, obsidian: 0, geode: 0, ore_bot: 1, clay_bot: 0, obsidian_bot: 0, geode_bot: 0 },
            RobotType::Clay,
        ));
        total_quality_level += geode_max as u32 * bp.id as u32;
    }
    println!("{}", total_quality_level);
}

fn search_next<const MAX_TIME: u16>(blueprint: &BluePrint, mut state: State, robot_next: RobotType) -> u16 {
    let resource_gathering_turns = match &robot_next {
        RobotType::Ore => {
            let shortage = blueprint.ore_bot_cost.0 as i32 - state.ore as i32;
            if shortage <= 0 { 0 } else { (shortage - 1) as u16 / state.ore_bot + 1 }
        },
        RobotType::Clay => {
            let shortage = blueprint.clay_bot_cost.0 as i32 - state.ore as i32;
            if shortage <= 0 { 0 } else { (shortage - 1) as u16 / state.ore_bot + 1 }
        },
        RobotType::Obsidian => {
            let shortage_ore = blueprint.obsidian_cost.0 as i32 - state.ore as i32;
            let shortage_clay = blueprint.obsidian_cost.1 as i32 - state.clay as i32;
            let ore_turn = if shortage_ore <= 0 { 0 } else { (shortage_ore - 1) as u16 / state.ore_bot + 1 };
            let clay_turn = if shortage_clay <= 0 { 0 } else { (shortage_clay - 1) as u16 / state.clay_bot + 1 };
            ore_turn.max(clay_turn)
        },
        RobotType::Geode => {
            let shortage_ore = blueprint.geode_bot_cost.0 as i32 - state.ore as i32;
            let shortage_obsi = blueprint.geode_bot_cost.1 as i32 - state.obsidian as i32;
            let ore_turn = if shortage_ore <= 0 { 0 } else { (shortage_ore - 1) as u16 / state.ore_bot + 1 };
            let obsi_turn = if shortage_obsi <= 0 { 0 } else { (shortage_obsi - 1) as u16 / state.obsidian_bot + 1 };
            ore_turn.max(obsi_turn)
        },
    };
    let time_inc = (resource_gathering_turns+1).min(MAX_TIME - state.time);
    state.ore += state.ore_bot * time_inc;
    state.clay += state.clay_bot * time_inc;
    state.obsidian += state.obsidian_bot * time_inc;
    state.geode += state.geode_bot * time_inc;
    state.time += time_inc;

    if state.time == MAX_TIME {
        return state.geode;
    }
    
    match &robot_next {
        RobotType::Ore => {
            state.ore -= blueprint.ore_bot_cost.0 as u16;
            state.ore_bot += 1;
        },
        RobotType::Clay => {
            state.ore -= blueprint.clay_bot_cost.0 as u16;
            state.clay_bot += 1;
        },
        RobotType::Obsidian => {
            state.ore -= blueprint.obsidian_cost.0 as u16;
            state.clay -= blueprint.obsidian_cost.1 as u16;
            state.obsidian_bot += 1;
        },
        RobotType::Geode => {
            state.ore -= blueprint.geode_bot_cost.0 as u16;
            state.obsidian -= blueprint.geode_bot_cost.1 as u16;
            state.geode_bot += 1;
        },
    }
    let mut max_geode: u16 = 0;
    assert!(state.ore_bot != 0);
    if state.ore_bot < blueprint.ore_consume_max as u16 {
        max_geode = max_geode.max(search_next::<MAX_TIME>(blueprint, state.clone(), RobotType::Ore));
    }
    let clay_consume_max = blueprint.obsidian_cost.1 as u16;
    if state.clay_bot < clay_consume_max {
        max_geode = max_geode.max(search_next::<MAX_TIME>(blueprint, state.clone(), RobotType::Clay));
    }
    let obsidian_consume_max = blueprint.geode_bot_cost.1 as u16;
    if state.obsidian_bot < obsidian_consume_max && state.clay_bot != 0 {
        max_geode = max_geode.max(search_next::<MAX_TIME>(blueprint, state.clone(), RobotType::Obsidian));
    }
    if state.obsidian_bot != 0 {
        max_geode = max_geode.max(search_next::<MAX_TIME>(blueprint, state.clone(), RobotType::Geode));
    }
    max_geode
}
