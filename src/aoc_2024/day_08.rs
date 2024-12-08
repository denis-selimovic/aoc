use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::plugin::Plugin;
use crate::reader::Reader;

pub struct AoC2024Day08;

fn in_range(point: &(i32, i32), m: i32, n: i32) -> bool {
    point.0 >= 0 && point.0 < m && point.1 >= 0 && point.1 < n
}

impl Plugin for AoC2024Day08 {
    fn execute(&self) -> (u64, u64) {
        let reader = Reader::new(8, 2024);
        let content = reader.load_puzzle();
        let mut positions: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

        let m = 50;
        let n = 50;

        for (i, line) in content.lines().into_iter().enumerate() {
            for (j, ch) in line.chars().into_iter().enumerate() {
                if !ch.is_alphanumeric() || ch == '.' {
                    continue;
                }

                positions.entry(ch).or_insert_with(Vec::new).push((i as i32, j as i32));
            }
        }

        let mut part1 = HashMap::new();
        let mut part2 = HashMap::new();

        for (ch, pos) in &positions {
            let pairs: Vec<_> = pos.iter().combinations(2).collect();

            for pair in pairs {
                let a = pair[0];
                let b = pair[1];

                let behind_a = (2 * a.0 - b.0, 2 * a.1 - b.1);
                let behind_b = (2 * b.0 - a.0, 2 * b.1 - a.1);

                if in_range(&behind_a, m, n) {
                    part1.entry(ch).or_insert_with(Vec::new).push(behind_a);
                }
                if in_range(&behind_b, m, n) {
                    part1.entry(ch).or_insert_with(Vec::new).push(behind_b);
                }
            }
        }

        for (ch, pos) in &positions {
            let pairs: Vec<_> = pos.iter().combinations(2).collect();

            for pair in pairs {
                let a = pair[0];
                let b = pair[1];
                let offset = (a.0 - b.0, a.1 - b.1);

                let mut pos = *a;

                while in_range(&pos, m, n) {
                    part2.entry(ch).or_insert_with(Vec::new).push(pos);
                    pos = (pos.0 + offset.0, pos.1 + offset.1);
                }

                pos = *b;

                while in_range(&pos, m, n) {
                    part2.entry(ch).or_insert_with(Vec::new).push(pos);
                    pos = (pos.0 - offset.0, pos.1 - offset.1);
                }
            }
        }

        let part1_res = part1
            .values()
            .flat_map(|v| v.iter().cloned())
            .collect::<HashSet<_>>()
            .len();

        let part2_res = part2
            .values()
            .flat_map(|v| v.iter().cloned())
            .collect::<HashSet<_>>()
            .len();


        (part1_res as u64, part2_res as u64)
    }
}
