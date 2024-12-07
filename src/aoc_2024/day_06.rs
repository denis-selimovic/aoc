use std::collections::HashSet;

use crate::plugin::Plugin;
use crate::reader::Reader;


pub struct AoC2024Day06;


fn in_range(x: i32, y: i32, m: i32, n: i32) -> bool {
    x >= 0 && x < m && y >= 0 && y < n
}

fn one_loop(start_x: i32, start_y: i32, start_dir: i32, grid: &Vec<Vec<String>>) -> Option<i32> {
    let m = grid.len();
    let n = grid[0].len();
    let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    let mut pos_x = start_x;
    let mut pos_y = start_y;
    let mut curr_dir = start_dir;
    let mut hits = HashSet::new();

    loop {
        if hits.contains(&(pos_x, pos_y, curr_dir)) {
            return None;
        }

        hits.insert((pos_x, pos_y, curr_dir));

        let dir = dirs[curr_dir as usize];
        let new_x = pos_x + dir.0;
        let new_y = pos_y + dir.1;

        if !in_range(new_x, new_y, m as i32, n as i32) {
            return Some(
                hits
                    .iter()
                    .map(|(i, j, _)| (*i, *j))
                    .collect::<HashSet<(i32, i32)>>()
                    .len() as i32
            )
        }

        if grid[new_x as usize][new_y as usize] == "#" {
            curr_dir = (curr_dir + 1) % 4;
        } else {
            pos_x = new_x;
            pos_y = new_y;
        }
    }

}


impl Plugin for AoC2024Day06 {
    fn execute(&self) -> (u64, u64) {
        let reader = Reader::new(6, 2024);
        let content = reader.load_puzzle();
        let mut grid = reader.to_grid(&content);

        let m = grid.len();
        let n = grid[0].len();

        let dirs = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
        let mut start_x = -1;
        let mut start_y = -1;
        

        for i in 0..m {
            for j in 0..n {
                if grid[i as usize][j as usize] == "^" {
                    start_x = i as i32;
                    start_y = j as i32;
                    break;
                }
            }

            if start_x != -1 || start_y != -1 {
                break;
            }
        }

        let start = (start_x, start_y);
        let part1 = one_loop(start_x, start_y, 0, &grid).unwrap_or(0) as u64;
        let mut part2: u64 = 0;

        let mut pos_x = start_x;
        let mut pos_y = start_y;
        let mut curr_dir = 0;
        let mut visited = HashSet::new();

        grid[start_x as usize][start_y as usize] = ".".to_string();

        loop {
            visited.insert((pos_x, pos_y));

            let new_x = pos_x + dirs[curr_dir].0;
            let new_y = pos_y + dirs[curr_dir].1;

            if !in_range(new_x, new_y, m as i32, n as i32) {
                break;
            }

            if grid[new_x as usize][new_y as usize] == "#" {
                curr_dir = (curr_dir + 1) % 4;
            } else {
                if (new_x, new_y) != start && !visited.contains(&(new_x, new_y)) {
                    grid[new_x as usize][new_y as usize] = "#".to_string();

                    if one_loop(pos_x, pos_y, curr_dir as i32, &grid).is_none() {
                        part2 += 1;
                    }

                    grid[new_x as usize][new_y as usize] = ".".to_string();
                }

                pos_x = new_x;
                pos_y = new_y;
            }
        }


        (part1, part2)
    }
}

