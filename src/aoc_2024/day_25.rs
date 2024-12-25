use itertools::Itertools;

use crate::plugin::Plugin;
use crate::reader::Reader;


pub struct AoC2024Day25;


fn parse_input(content: &str) -> Vec<u64> {
    content
        .split("\n\n")
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| {
            s
                .chars()
                .filter(char::is_ascii_punctuation)
                .map(|c| c as u64)
                .fold(0, |acc, c| (c & 1) | (acc << 1))
        })
        .collect_vec()
}


impl Plugin for AoC2024Day25 {
    fn execute(&self) -> (u64, u64) {
        let reader = Reader::new(25, 2024);
        let puzzle = reader.load_puzzle();
        let input = parse_input(&puzzle);

        let part1 = input
            .iter()
            .copied()
            .tuple_combinations()
            .filter(|(a, b)| a & b == 0)
            .count();

        (part1 as u64, 0)
    }
}
