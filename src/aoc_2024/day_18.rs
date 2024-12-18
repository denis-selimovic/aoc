use std::collections::HashMap;
use std::u64;

use crate::plugin::Plugin;
use crate::reader::Reader;


pub struct AoC2024Day18;

fn parse_bytes(content: &String) -> Vec<(i32, i32)> {
    content
        .lines()
        .into_iter()
        .map(|line| {
            let spl = line.split(",").map(|s| s.to_string()).collect::<Vec<String>>();
            let x = spl[0].parse::<i32>().unwrap();
            let y = spl[1].parse::<i32>().unwrap();

            (x, y)
        })
        .collect::<Vec<_>>()
}

fn make_grid(bytes: &Vec<(i32, i32)>, m: usize, n: usize, take: usize) -> Vec<Vec<u8>> {
    let mut grid = vec![vec![1; n]; m];

    for (y, x) in bytes.into_iter().take(take) {
        grid[*x as usize][*y as usize] = 0;
    }

    grid
}

fn in_range(pos: (i32, i32), range: (i32, i32)) -> bool {
    pos.0 >= 0 && pos.0 < range.0 && pos.1 >= 0 && pos.1 < range.1
}

fn min_steps(grid: &Vec<Vec<u8>>, start: (i32, i32), end: (i32, i32)) -> Option<u64> {
    let dirs = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    let range = (grid.len() as i32, grid[0].len() as i32);

    let mut todo = vec![(start, 0)];
    let mut seen = HashMap::new();
    seen.insert(start, 0);

    while todo.len() > 0 {
        let mut next = Vec::new();

        for (pos, steps) in todo {
            for dir in &dirs {
                let nxt = (pos.0 + dir.0, pos.1 + dir.1);

                if !in_range(nxt, range) {
                    continue;
                }

                if grid[nxt.0 as usize][nxt.1 as usize] == 0 {
                    continue;
                }
                
                if steps + 1 >= *seen.entry(nxt).or_insert(u64::MAX) {
                    continue;
                }
               
                next.push((nxt, steps + 1));
                seen.insert(nxt, steps + 1);
            }
        }

        todo = next;
    }

    match seen.get(&end) {
        Some(x) => Some(*x),
        None => None,
    }
}

fn bin_search(bytes: &Vec<(i32, i32)>, start: (i32, i32), end: (i32, i32), begin: usize, stop: usize) -> usize {
    let (mut low, mut high) = (begin, stop);

    while low < high {
        let mid = low + (high - low) / 2;
        let grid = make_grid(bytes, 71, 71, mid);

        if min_steps(&grid, start, end).is_none() {
            high = mid;
        } else {
            low = mid + 1;
        }
    }

    low
}

impl Plugin for AoC2024Day18 {
    fn execute(&self) -> (u64, u64) {
        let reader = Reader::new(18, 2024);
        let content = reader.load_puzzle();

        let bytes = parse_bytes(&content);
        let grid = make_grid(&bytes, 71, 71, 1024);
        let part1 = min_steps(&grid, (0, 0), (70, 70)).unwrap();
        let part2 = bin_search(&bytes, (0, 0), (70, 70), 0, bytes.len());

        println!("Part2 solution: {},{}", bytes[part2 - 1].0, bytes[part2 - 1].1);

        (part1, part2 as u64)
    }
}
