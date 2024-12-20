use std::collections::HashMap;

use itertools::Itertools;
use num_complex::Complex;

use crate::plugin::Plugin;
use crate::reader::Reader;


pub struct AoC2024Day20;

fn make_grid(content: &String) -> HashMap<Complex<i32>, char> {
    content
        .lines()
        .into_iter()
        .enumerate()
        .map(|(i, line)| {
            line
                .chars()
                .enumerate()
                .filter_map(|(j, ch)| {
                    if ch == '#' {
                        None
                    } else {
                        Some((Complex::new(i as i32, j as i32), ch))
                    }
                })
                .collect::<Vec<_>>()
        })
        .flat_map(|it| it.into_iter())
        .collect::<HashMap<_, _>>()
}

fn find_start(grid: &HashMap<Complex<i32>, char>) -> Complex<i32> {
    for (key, val) in grid.iter() {
        if *val == 'S' {
            return *key;
        }
    }

    panic!("No start position");
}

fn get_neighbours(curr: Complex<i32>) -> Vec<Complex<i32>> {
    vec![
        curr + Complex::new(-1, 0),
        curr + Complex::new(1, 0),
        curr + Complex::new(0, -1),
        curr + Complex::new(0, 1),
    ]
}

fn bfs(grid: &HashMap<Complex<i32>, char>, start: Complex<i32>) -> HashMap<Complex<i32>, i32> {
    let mut dist = HashMap::new();
    let mut todo = vec![(start, 0)];
    dist.insert(start, 0);

    while todo.len() > 0 {
        let mut nxt = Vec::new();

        for (pos, curr) in &todo {
            for ngh in get_neighbours(*pos) {
                if grid.contains_key(&ngh) && !dist.contains_key(&ngh) {
                    dist.insert(ngh, curr + 1);
                    nxt.push((ngh, curr + 1));
                }
            }
        }

        todo = nxt;
    }

    dist
}


impl Plugin for AoC2024Day20 {
    fn execute(&self) -> (u64, u64) {
        let reader = Reader::new(20, 2024);
        let puzzle = reader.load_puzzle();

        let grid = make_grid(&puzzle);
        let start = find_start(&grid);
        let distances = bfs(&grid, start);

        let mut part1 = 0;
        let mut part2 = 0;

        println!("{:?}", start);
        println!("{}", distances.len());

        let combs = distances
            .into_iter()
            .combinations(2)
            .map(|pair| (pair[0], pair[1]))
            .collect::<Vec<_>>();

        for ((x, i), (y, j)) in combs {
            let dist = (x - y).re.abs() + (x - y).im.abs();

            if dist == 2 && j - i - dist >= 100 {
                part1 += 1;
            }

            if dist < 21 && j - i - dist >= 100 {
                part2 += 1;
            }
        }

        (part1, part2)
    }
}
