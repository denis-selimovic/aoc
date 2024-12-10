use std::collections::{HashMap, HashSet, VecDeque};

use crate::plugin::Plugin;
use crate::reader::Reader;


pub struct AoC2024Day10;

fn make_grid(puzzle: &Vec<Vec<String>>) -> Vec<Vec<i32>> {
    puzzle
        .into_iter()
        .map(|row| {
            row
                .into_iter()
                .map(|e| e.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect()
}

fn find_trailheads(grid: &Vec<Vec<i32>>) -> VecDeque<(i32, i32, i32, i32)> {
    grid
        .into_iter()
        .enumerate()
        .map(|(i, row)| {
            row
                .into_iter()
                .enumerate()
                .filter(|(_, v)| **v == 0)
                .map(|(j, _)| (i as i32, j as i32, i as i32, j as i32))
                .collect::<Vec<(i32, i32, i32, i32)>>()
        })
        .flat_map(|row| row)
        .collect()
}

fn in_range(x: i32, y: i32, m: i32, n: i32) -> bool {
    x >= 0 && x < m && y >= 0 && y < n
}

impl Plugin for AoC2024Day10 {
    fn execute(&self) -> (u64, u64) {
        let reader = Reader::new(10, 2024);
        let content = reader.load_puzzle();
        let puzzle = reader.to_grid(&content);

        let grid = make_grid(&puzzle);
        let mut trailheads = find_trailheads(&grid);
        let mut trailheads_new = trailheads.clone();

        let (m, n) = (grid.len() as i32, grid[0].len() as i32);
        let dirs = vec![(0, -1), (-1, 0), (1, 0), (0, 1)];
        let mut mp: HashMap<(i32, i32), i32> = HashMap::new();
        let mut mp2: HashMap<(i32, i32), i32> = HashMap::new();

        while !trailheads.is_empty() {
            let mut new_trailheads = VecDeque::new();
            let mut seen = HashSet::new();

            for entry in &trailheads {
                if grid[entry.0 as usize][entry.1 as usize] == 9 {
                    *mp.entry((entry.0, entry.1)).or_insert(0) += 1;
                    continue;
                }

                for dir in &dirs {
                    let x = entry.0 + dir.0;
                    let y = entry.1 + dir.1;

                    if in_range(x, y, m, n) && grid[x as usize][y as usize] - grid[entry.0 as usize][entry.1 as usize] == 1 {
                        if !seen.contains(&(x, y, entry.2, entry.3)) {
                            new_trailheads.push_back((x, y, entry.2, entry.3));
                            seen.insert((x, y, entry.2, entry.3));
                        }
                    }
                }
            }
            
            trailheads = new_trailheads;
        }

        while !trailheads_new.is_empty() {
            let mut new_trailheads = VecDeque::new();

            for entry in &trailheads_new {
                if grid[entry.0 as usize][entry.1 as usize] == 9 {
                    *mp2.entry((entry.2, entry.3)).or_insert(0) += 1;
                    continue;
                }

                for dir in &dirs {
                    let x = entry.0 + dir.0;
                    let y = entry.1 + dir.1;
                
                    if in_range(x, y, m, n) && grid[x as usize][y as usize] - grid[entry.0 as usize][entry.1 as usize] == 1 {
                        new_trailheads.push_back((x, y, entry.2, entry.3));
                    }
                }
            }

            trailheads_new = new_trailheads;
        }

        let part1: u64 = mp
            .values()
            .into_iter()
            .map(|v| *v as u64)
            .sum();
        
        let part2: u64 = mp2
            .values()
            .into_iter()
            .map(|v| *v as u64)
            .sum();

        (part1, part2)
    }
}
