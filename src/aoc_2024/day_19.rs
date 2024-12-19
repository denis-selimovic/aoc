use std::collections::HashMap;

use crate::plugin::Plugin;
use crate::reader::Reader;


pub struct AoC2024Day19;

fn parse_input(content: &String) -> (Vec<String>, Vec<String>) {
    let mut iter = content.lines().into_iter();

    let patterns = if let Some(pattern_line) = iter.next() {
        pattern_line.split(", ").map(|s| s.to_string()).collect::<Vec<String>>()
    } else {
        panic!("Wrong file format");
    };

    iter.next().unwrap();

    let designs = iter
        .map(|s| s.to_string())
        .collect();

    (patterns, designs)
}

fn possible_count(idx: usize, design: &String, patterns: &Vec<String>, cache: &mut HashMap<(String, usize), u64>) -> u64 {
    if idx == design.len() {
        return 1;
    }

    if idx > design.len() {
        return 0;
    }

    if let Some(val) = cache.get(&(design.clone(), idx)) {
        return *val;
    }

    let substr = &design[idx..];

    let applicable = patterns
        .into_iter()
        .filter(|p| substr.starts_with(*p))
        .map(|p| p.to_string())
        .collect::<Vec<String>>();

    if applicable.len() == 0 {
        cache.insert((design.clone(), idx), 0);

        return 0;
    }

    let ans: u64 = applicable
        .into_iter()
        .map(|p| possible_count(idx + p.len(), design, patterns, cache))
        .sum();

    cache.insert((design.clone(), idx), ans);

    ans
}

impl Plugin for AoC2024Day19 {
    fn execute(&self) -> (u64, u64) {
        let reader = Reader::new(19, 2024);
        let puzzle = reader.load_puzzle();

        let (patterns, designs) = parse_input(&puzzle);
        let mut cache = HashMap::new();

        let pcount = designs
            .into_iter()
            .map(|d| possible_count(0, &d, &patterns, &mut cache))
            .collect::<Vec<_>>();
        
        let part1 = pcount.clone().into_iter().filter(|c| *c > 0).collect::<Vec<_>>().len();
        let part2: u64 = pcount.into_iter().sum();

        (part1 as u64, part2)
    }
}
