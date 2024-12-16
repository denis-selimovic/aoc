use std::collections::{BinaryHeap, HashSet};
use std::usize;

use crate::plugin::Plugin;
use crate::reader::Reader;


pub struct AoC2024Day16;


fn make_grid(content: &String) -> Vec<Vec<char>> {
    content
        .lines()
        .into_iter()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect()
}

fn find_path(grid: &Vec<Vec<char>>) -> ((i32, i32), (i32, i32)) {
    let mut start = (-1, -1);
    let mut end = (-1, -1);

    for (i, row) in grid.into_iter().enumerate() {
        for (j, ch) in row.into_iter().enumerate() {
            if *ch == 'S' {
                start = (i as i32, j as i32);
            }
            else if *ch == 'E' {
                end = (i as i32, j as i32);
            }
        }
    }

    (start, end)
}

fn in_range(x: i32, y: i32, m: i32, n: i32) -> bool {
    x >= 0 && x < m && y >= 0 && y < n
}


impl Plugin for AoC2024Day16 {
    fn execute(&self) -> (u64, u64) {
        let reader = Reader::new(16, 2024);
        let puzzle = reader.load_puzzle();

        let dirs = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
        let grid = make_grid(&puzzle);
        let (m, n) = (grid.len(), grid[0].len());
        let (start, end) = find_path(&grid);

        let mut distances = vec![vec![usize::MAX; n]; m];
        let mut heap = BinaryHeap::new();

        let mut seen: HashSet<(i32, i32)> = HashSet::new();
        let mut best = usize::MAX;

        distances[start.0 as usize][start.1 as usize] = 0;
        heap.push((0 as i32, start.0, start.1, (0, 1), vec![start]));


        while let Some((cost, i, j, dir, path)) = heap.pop() {
            let cost = cost as usize;

            if cost > distances[i as usize][j as usize] {
                continue;
            } else {
                distances[i as usize][j as usize] = cost;
            }

            if grid[i as usize][j as usize] == 'E' && cost <= best {
                best = cost;
                
                for val in &path {
                    seen.insert((val.0, val.1));
                }
            }

            for cdir in &dirs {
                let x = i + cdir.0;
                let y = j + cdir.1;


                if !in_range(x as i32, y as i32, m as i32, n as i32) {
                    continue;
                }

                if grid[x as usize][y as usize] == '#' {
                    continue;
                }

                let next_cost = cost + if dir == *cdir {
                    1
                } else {
                    1001
                };


                let mut npath = path.clone();
                npath.push((x, y));
                heap.push((next_cost as i32, x, y, *cdir, npath));
            }
        }

        (distances[end.0 as usize][end.1 as usize] as u64, seen.len() as u64)
    }
}
