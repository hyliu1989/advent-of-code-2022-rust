extern crate ndarray;
extern crate num;
extern crate pathfinding;

use ndarray::{Array2, NewAxis};
use pathfinding::prelude::bfs;


struct EvolvingMaze {
    blizzards: Vec<(usize, usize, usize, usize)>,
    mazes: Vec<Array2<u32>>,
    shape: (usize, usize),
}

fn build_maze(data: &[u8]) -> EvolvingMaze {
    let m = 
        data.split(|b| *b == b'\n')
            .filter(|l| l.len() != 0)
            .count() - 2;
    let n = 
        data.split(|b| *b == b'\n')
            .next()
            .unwrap()
            .len() - 2;
    let mut blizzards: Vec::<(usize, usize, usize, usize)> = Vec::new();
    for (i, line) in data.split(|b| *b == b'\n').enumerate() {
        for (j, c) in line.iter().enumerate() {
            match c {
                b'<' => {blizzards.push((i-1, j-1, 0, n-1))},
                b'>' => {blizzards.push((i-1, j-1, 0, 1))},
                b'^' => {blizzards.push((i-1, j-1, m-1, 0))},
                b'v' => {blizzards.push((i-1, j-1, 1, 0))},
                b'.' | b'#' => {},
                _ => panic!("Invalid input"),
            };
        }
    }
    let num_mazes = num::integer::lcm(m, n);
    let mut mazes = Vec::with_capacity(num_mazes);
    for idx_evolution in 0..num_mazes {
        mazes.push(Array2::<u32>::zeros((m, n)));
        for (i, j, di, dj) in blizzards.iter() {
            let i = (i + idx_evolution * di) % m;
            let j = (j + idx_evolution * dj) % n;
            mazes[idx_evolution][[i, j]] += 1;
        }
    }
    EvolvingMaze {blizzards, mazes, shape: (m, n)}
}


#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(isize, usize, isize);

fn main() {
    println!("Hello, world!");
    let data = include_bytes!("../input.txt");
    let mazes = build_maze(data);
    println!("mazes.shape: {:?}", mazes.shape);
    let successors = |p: &Pos| {
        let idx_evolution = (p.2 + 1) % mazes.mazes.len() as isize;
        let current_maze = &mazes.mazes[idx_evolution as usize];
        let (m, n) = &mazes.shape;
        let mut successors: Vec<Pos> = Vec::new();
        if p.0 == -1 {
            successors.push(Pos(p.0, p.1, idx_evolution));
            if current_maze[[0, 0]] == 0 {
                successors.push(Pos(0, 0, idx_evolution));
            }
        } else if p.0 == *m as isize {
            successors.push(Pos(p.0, p.1, idx_evolution));
            if current_maze[[m - 1, n - 1]] == 0 {
                successors.push(Pos((m - 1) as isize, n - 1, idx_evolution));
            }
        } else {
            if current_maze[[p.0 as usize, p.1]] == 0 {
                successors.push(Pos(p.0, p.1, idx_evolution));
            }
            if (p.0 as usize) > 0 && current_maze[[(p.0 - 1) as usize, p.1]] == 0 {
                successors.push(Pos(p.0 - 1, p.1, idx_evolution));
            }
            if (p.0 as usize) < m - 1 && current_maze[[(p.0 + 1) as usize, p.1]] == 0 {
                successors.push(Pos(p.0 + 1, p.1, idx_evolution));
            }
            if p.1 > 0 && current_maze[[p.0 as usize, p.1 - 1]] == 0 {
                successors.push(Pos(p.0, p.1 - 1, idx_evolution));
            }
            if p.1 < n - 1 && current_maze[[p.0 as usize, p.1 + 1]] == 0 {
                successors.push(Pos(p.0, p.1 + 1, idx_evolution));
            }
        }
        successors
    };
    let reached = |p: &Pos| { 
        ((p.0 as usize) == mazes.shape.0 - 1) && (p.1 == mazes.shape.1 - 1)
    };
    let result = bfs(&Pos(-1, 0, 0), successors, reached).unwrap();
    // println!("Result: {:?}", result);
    println!("Part 1 len: {}", result.len());

    let new_start_1 = {
        let last = result.iter().rev().next().unwrap();
        Pos(last.0 + 1, last.1, last.2 + 1)
    };
    let reached_rev = |p: &Pos| { 
        ((p.0 as usize) == 0) && (p.1 == 0)
    };
    println!("New start: {:?}", new_start_1);
    let result_rev = bfs(&new_start_1, successors, reached_rev).unwrap();

    let new_start_2 = {
        let last = result_rev.iter().rev().next().unwrap();
        Pos(last.0 - 1, last.1, last.2 + 1)
    };
    let result2 = bfs(&new_start_2, successors, reached).unwrap();

    println!("Part 2 len: {}", result.len() + result_rev.len() + result2.len());
}
