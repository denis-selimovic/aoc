use core::panic;
use std::collections::HashMap;

use crate::plugin::Plugin;
use crate::reader::Reader;


pub struct AoC2024Day15;

fn parse(content: &String) -> (Vec<Vec<char>>, Vec<char>) {
    let mut grid = Vec::new();
    let mut commands = Vec::new();
    let mut is_grid = true;

    for line in content.lines() {
        if line.len() == 0 {
            is_grid = false;
            continue;
        }

        if is_grid {
            grid.push(line.chars().collect());
        } else {
            commands.extend(line.trim_end().chars().collect::<Vec<_>>());
        }
    }

    (grid, commands)
}

fn extend_grid(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut res = Vec::new();

    for row in grid {
        let mut new_row = Vec::new();

        for ch in row {
            if *ch == '#' {
                new_row.push('#');
                new_row.push('#');
            } else if *ch == 'O' {
                new_row.push('[');
                new_row.push(']');
            } else if *ch == '@' {
                new_row.push('@');
                new_row.push('.');
            } else if *ch == '.' {
                new_row.push('.');
                new_row.push('.');
            } else {
                panic!("Wrong char");
            }
        }

        res.push(new_row);
    }

    res
}

fn find_start(grid: &Vec<Vec<char>>) -> (i32, i32) {
    for (i, row) in grid.into_iter().enumerate() {
        for (j, ch) in row.into_iter().enumerate() {
            if *ch == '@' {
                return (i as i32, j as i32);
            }
        }
    }

    (-1, -1)
}

fn in_range(x: i32, y: i32, m: i32, n: i32) -> bool {
    x >= 0 && x < m && y >= 0 && y < n
}

fn try_box_push(grid: &Vec<Vec<char>>, cx: i32, cy: i32, m: i32, n: i32, dir: &(i32, i32)) -> Option<(i32, i32)> {
    let (mut tx, mut ty) = (cx, cy);

    while in_range(tx, ty, m, n) {
        if grid[tx as usize][ty as usize] == '.' {
            return Some((tx, ty))
        } else if grid[tx as usize][ty as usize] == '#' {
            return None;
        } else if grid[tx as usize][ty as usize] == 'O' {
            tx += dir.0;
            ty += dir.1;
        } else {
            panic!("Unknown char while moving boxes!");
        }
    }

    None
}

fn try_box_push_horizontally(grid: &mut Vec<Vec<char>>, cx: i32, cy: i32, m: i32, n: i32, dir: &(i32, i32)) -> Option<(i32, i32)> {
    let (mut tx, mut ty) = (cx, cy);
    
    while in_range(tx, ty, m, n) {
        let val = grid[tx as usize][ty as usize];

        if val == '#' {
            return None;
        } else if val == '[' || val == ']' {
            tx += dir.0;
            ty += dir.1;
        } else if val == '.' {
            if dir.1 == -1 {
                for j in ty..=cy {
                    grid[cx as usize][j as usize] = grid[cx as usize][(j + 1) as usize];
                }
            } else if dir.1 == 1  {
                for j in (cy..=ty).rev() {
                    grid[cx as usize][j as usize] = grid[cx as usize][(j - 1) as usize]; 
                }
            } else {
                panic!("Wrong direction for part 2");
            }

            return Some((tx, ty));
        } else {
            panic!("Unknown char while moving boxes in part2!");
        }
    }

    None
}

fn try_box_push_vertically(grid: &mut Vec<Vec<char>>, cx: i32, cy: i32, m: i32, _n: i32, dir: &(i32, i32)) -> Option<(i32, i32)> {
    let (mut tx, _ty)= (cx, cy);
    let mut mp: HashMap<i32, (i32, i32)> = HashMap::new();
    mp.insert(cx, (cy, cy));


    while tx >= 0 && tx < m {
        let (left, right) = mp.get(&tx).unwrap();
        let mut min_left = (*left) as i32;
        let mut max_right = (*right) as i32;

        tx += dir.0;

        if tx < 0 || tx >= m {
            return None;
        }

        let row = &grid[tx as usize];
        let slice = &row[(*left as usize)..((*right as usize) + 1)];

        if slice.iter().all(|&x| x == '.') {
            break;
        }
        if slice.iter().any(|&x| x == '#') {
            return None;
        }

        if row[(*left) as usize] == ']' {
            min_left -= 1;
        } else {
            min_left = *left + slice.iter().position(|&x| x == '[').unwrap() as i32; 
        }

        if row[(*right) as usize] == '[' {
            max_right += 1;
        } else {
            max_right = *left + slice.iter().rposition(|&x| x == ']').unwrap() as i32;
        }
        mp.insert(tx, (min_left, max_right));
    }

    if dir.0 == -1 {
        let mut keys = mp.keys().into_iter().collect::<Vec<_>>();
        keys.sort();

        for key in keys {
            let (l, r) = mp.get(key).unwrap();
            let (ll, rr) = mp.get(&(key - 1)).unwrap_or(&(1000000, -1));

            let start = std::cmp::min(l, ll);
            let end = std::cmp::max(r, rr);

            for idx in *start..=*end {
                if idx >= *l && idx <= *r {
                    grid[(*key - 1) as usize][idx as usize] = grid[*key as usize][idx as usize];
                } else {
                    grid[(*key - 1) as usize][idx as usize] = '.';
                }
            }
        }
    } else if dir.0 == 1 {
        let mut keys = mp.keys().into_iter().collect::<Vec<_>>();
        keys.sort();
        keys.reverse();

        for key in keys {
            let (l, r) = mp.get(key).unwrap();
            let (ll, rr) = mp.get(&(key + 1)).unwrap_or(&(1000000, -1));

            let start = std::cmp::min(l, ll);
            let end = std::cmp::max(r, rr);

            for idx in *start..=*end {
                if idx >= *l && idx <= *r {
                    grid[(*key + 1) as usize][idx as usize] = grid[*key as usize][idx as usize];
                } else {
                    grid[(*key + 1) as usize][idx as usize] = '.';
                }
            }
        }
    } else {
        panic!("Wrong direction for moving vertically in part2");
    }

    Some((cx, cy)) 
}

fn one_move(grid: &mut Vec<Vec<char>>, dirs: &HashMap<char, (i32, i32)>, command: char, cx: i32, cy: i32) -> (i32, i32) {
    let cdir = dirs.get(&command).unwrap();
    let (nx, ny) = (cx + cdir.0, cy + cdir.1);
    let (m, n) = (grid.len() as i32, grid[0].len() as i32);

    if !in_range(nx, ny, m, n) {
        return (cx, cy);
    }

    if grid[nx as usize][ny as usize] == '.' {
        grid[cx as usize][cy as usize] = '.';
        grid[nx as usize][ny as usize] = '@';

        return (nx, ny);
    } else if grid[nx as usize][ny as usize] == '#' {
        return (cx, cy);
    } else if grid[nx as usize][ny as usize] == 'O' {
        if let Some((bx, by)) = try_box_push(&grid, nx, ny, m, n, cdir) {
            grid[bx as usize][by as usize] = 'O';
            grid[nx as usize][ny as usize] = '@';
            grid[cx as usize][cy as usize] = '.';

            return (nx, ny);
        } else {
            return (cx, cy);
        }
    } else if grid[nx as usize][ny as usize] == '[' || grid[nx as usize][ny as usize] == ']' {
        if command == '<' || command == '>' {
            match try_box_push_horizontally(grid, nx, ny, m, n, cdir) {
                Some(_) => {
                    grid[cx as usize][cy as usize] = '.';
                    return (nx, ny);
                },
                None => return (cx, cy)
            };
        } else {
            match try_box_push_vertically(grid, cx, cy, m, n, cdir) {
                Some(_) => {
                    grid[cx as usize][cy as usize] = '.';
                    return (nx, ny);
                },
                None => return (cx, cy)
            };
        }
    } else {
        panic!("Should never be reached. Unknown char at position in grid");
    }
}

impl Plugin for AoC2024Day15 {
    fn execute(&self) -> (u64, u64) {
        let reader = Reader::new(15, 2024);
        let puzzle = reader.load_puzzle();

        let (mut grid, commands) = parse(&puzzle);
        let mut new_grid = extend_grid(&grid);
        let dirs = HashMap::from([
            ('^', (-1, 0)),
            ('>', (0, 1)),
            ('v', (1, 0)),
            ('<', (0, -1)),
        ]);

        let (mut sx, mut sy) = find_start(&grid);
        let (mut s2x, mut s2y) = find_start(&new_grid);
        
        for command in &commands {
            (sx, sy) = one_move(&mut grid, &dirs, *command, sx, sy);
            (s2x, s2y) = one_move(&mut new_grid, &dirs, *command, s2x, s2y);
        }

        let part1: u64 = grid
            .into_iter()
            .enumerate()
            .map(|(i, row)| {
                row
                    .into_iter()
                    .enumerate()
                    .map(|(j, ch)| (i, j, ch))
                    .collect::<Vec<(usize, usize, char)>>()
            })
            .flat_map(|v| v.into_iter())
            .map(|(i, j, ch)| {
                if ch == 'O' {
                    return (100 * i + j) as u64;
                } else {
                    return 0;
                }
            })
            .sum();

        let part2: u64 = new_grid
            .into_iter()
            .enumerate()
            .map(|(i, row)| {
                row
                    .into_iter()
                    .enumerate()
                    .map(|(j, ch)| (i, j, ch))
                    .collect::<Vec<(usize, usize, char)>>()
            })
            .flat_map(|v| v.into_iter())
            .map(|(i, j, ch)| {
                if ch == '[' {
                    return (100 * i + j) as u64;
                } else {
                    return 0;
                }
            })
            .sum();

        (part1, part2)
    }
}
