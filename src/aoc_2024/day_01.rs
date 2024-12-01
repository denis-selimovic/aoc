use std::collections::HashMap;

use crate::plugin::Plugin;
use crate::reader::Reader;

pub struct AoC2024Day01;

impl Plugin for AoC2024Day01 {
    fn execute(&self) -> (u64, u64) {
        let reader = Reader::new(1, 2024);
        let puzzle = reader.load_puzzle();
        let values = reader.split_vertically(&puzzle, "   ", 2);

        let mut left: Vec<u32> = values[0].iter().map(|l| l.parse::<u32>().unwrap()).collect();
        let mut right: Vec<u32> = values[1].iter().map(|r| r.parse::<u32>().unwrap()).collect();

        left.sort();
        right.sort();

        let occurences = right.iter().fold(HashMap::new(), |mut acc, &num| {
            *acc.entry(num).or_insert(0) += 1;
            acc
        });

        let part1: u64 = left
            .iter()
            .zip(right.iter())
            .map(|(l, r)| l.abs_diff(*r) as u64)
            .sum();
        let part2: u64 = left
            .iter()
            .map(|l| (l * occurences.get(l).unwrap_or(&0)) as u64)
            .sum();


        (part1, part2) 
    }
}
