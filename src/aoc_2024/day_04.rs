use std::collections::HashMap;

use crate::plugin::Plugin;
use crate::reader::Reader;

struct Counter<T> {
    counts: HashMap<T, usize>,
}

impl<T: Eq + std::hash::Hash> Counter<T> {
    fn new() -> Self {
        Counter {
            counts: HashMap::new(),
        }
    }

    fn add(&mut self, item: T) {
        *self.counts.entry(item).or_insert(0) += 1;
    }

    fn get(&self, item: &T) -> usize {
        *self.counts.get(item).unwrap_or(&0)
    }
}

pub struct AoC2024Day04;

fn in_range(x: i32, y: i32, m: i32, n: i32) -> bool {
    return x >= 0 && x < m && y >= 0 && y < n;
}


fn dfs_part1(content: &Vec<Vec<String>>, x: i32, y: i32, idx: usize, dirs: (i32, i32), strs: &Vec<String>) -> u64 {
    if idx == 3 {
        return 1;
    }

    let mut ans = 0;

    let new_x = x + dirs.0;
    let new_y = y + dirs.1;

    if new_x >= 0 && new_x < content.len() as i32 {
        if new_y >= 0 && new_y < content[new_x as usize].len() as i32 {
            if content[new_x as usize][new_y as usize] == strs[idx + 1] {
                ans += dfs_part1(content, new_x, new_y, idx + 1, dirs, strs)
            }
        }
    }

    ans
} 

fn dfs_part2(content: &Vec<Vec<String>>, x: i32, y: i32, m: i32, n: i32) -> u64 {
    let lux = x - 1;
    let luy = y - 1;

    let ldx = x + 1;
    let ldy = y - 1;

    let rux = x - 1;
    let ruy = y + 1;

    let rdx = x + 1;
    let rdy = y + 1;

    if !in_range(lux, luy, m, n) || !in_range(ldx, ldy, m, n) || !in_range(rux, ruy, m, n) || !in_range(rdx, rdy, m, n) {
        return 0;
    }

    if content[lux as usize][luy as usize] == content[rdx as usize][rdy as usize] {
        return 0;
    }
    if content[ldx as usize][ldy as usize] == content[rux as usize][ruy as usize] {
        return 0;
    }
    
    let mut counter = Counter::new();
    counter.add(content[lux as usize][luy as usize].clone());
    counter.add(content[ldx as usize][ldy as usize].clone());
    counter.add(content[rdx as usize][rdy as usize].clone());
    counter.add(content[rux as usize][ruy as usize].clone());


    if counter.get(&"M".to_string()) == 2 && counter.get(&"S".to_string()) == 2 {
        return 1;
    } else {
        return 0;
    }
}

impl Plugin for AoC2024Day04 {
    fn execute(&self) -> (u64, u64) {
        let dirs = vec![(-1, 0), (1, 0), (0, -1), (0, 1), (1, 1), (1, -1), (-1, 1), (-1, -1)];
        let strs = vec!["X".to_string(), "M".to_string(), "A".to_string(), "S".to_string()];
        let reader = Reader::new(4, 2024);
        let content = reader.load_puzzle();
        let puzzle = reader.split(&content, "");

        let mut starts = Vec::new();
        let mut starts2 = Vec::new();

        for i in 0..puzzle.len() {
            for j in 0..puzzle[i].len() {
                if puzzle[i][j] == "X" {
                    starts.push((i as i32, j as i32));
                } else if puzzle[i][j] == "A" {
                    starts2.push((i as i32, j as i32));
                }
            }
        }

        let part1: u64 = starts
            .into_iter()
            .map(|(i, j)| {
                let mut ans = 0;

                for dir in &dirs {
                    ans += dfs_part1(&puzzle, i, j, 0, *dir, &strs)
                }

                ans
            })
            .sum();

        let part2: u64 = starts2
            .into_iter()
            .map(|(i, j)| dfs_part2(&puzzle, i, j, puzzle.len() as i32, puzzle[0].len() as i32))
            .sum();

        (part1, part2)
    }
}
