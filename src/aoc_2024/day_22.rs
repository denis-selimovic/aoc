use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::plugin::Plugin;
use crate::reader::Reader;


pub struct AoC2024Day22;


fn parse_input(content: &String) -> Vec<u128> {
    content
        .lines()
        .map(|l| l.to_string().parse::<u128>().unwrap())
        .collect()
}

fn one_step(secret: u128) -> u128 {
    let mut val = secret;

    val ^= (val << 0x6) & 0xFFFFFF;
    val ^= (val >> 0x5) & 0xFFFFFF;
    val ^= (val << 0xB) & 0xFFFFFF;

    val
}

fn n_steps(val: u128, n: usize) -> u128 {
    (0..n).fold(val, |acc, _| one_step(acc))
}


impl Plugin for AoC2024Day22 {
    fn execute(&self) -> (u64, u64) {
        let reader = Reader::new(22, 2024);
        let puzzle = reader.load_puzzle();
        let inputs = parse_input(&puzzle);

        let part1: u128 = inputs
            .clone()
            .into_iter()
            .map(|num| n_steps(num, 2000))
            .sum();

        let mut profitmap: HashMap<(i128, i128, i128, i128), i128> = HashMap::new();

        for num in inputs {
            let mut prices = [0; 2000];
            let mut entry = num;

            for price in prices.iter_mut() {
                entry = one_step(entry);
                *price = (entry % 10) as i128;
            }

            let mut seen = HashSet::new();

            for (a, b, c, d, e) in prices.iter().tuple_windows() {
                let diffs = ((b - a), (c - b), (d - c), (e - d));

                if seen.insert(diffs) {
                    *profitmap.entry(diffs).or_default() += e;
                }
            }
        }

        let part2 = profitmap
            .into_values()
            .max()
            .unwrap();

        (part1 as u64, part2 as u64) 
    }
}
