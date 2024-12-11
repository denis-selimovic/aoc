use std::collections::HashMap;

use crate::plugin::Plugin;
use crate::reader::Reader;


pub struct AoC2024Day11;

fn blink(n: u64, stone: u64, cache: &mut HashMap<(u64, u64),u64>) -> u64 {
    let res = if n == 0 {
        1
    } else if let Some(cached) = cache.get(&(n, stone)) {
        *cached
    } else if stone == 0 {
        blink(n - 1, 1, cache)
    } else {
        let digits = (stone as f64).log10().floor() as u64 + 1;

        if digits % 2 == 0 {
            let factor = 10_usize.pow(digits as u32 / 2) as u64;
            let left = stone / factor;
            let right = stone % factor;
            
            blink(n - 1, left, cache) + blink(n - 1, right, cache)
        } else {
            blink(n - 1, 2024 *stone, cache)
        }
    };

    cache.insert((n, stone), res);

    res
}

impl Plugin for AoC2024Day11 {
    fn execute(&self) -> (u64, u64) {
        let reader = Reader::new(11, 2024);
        let puzzle = reader.load_puzzle();
        let inputs = puzzle
            .trim_end()
            .split(" ")
            .into_iter()
            .map(|n| n.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        let mut cache: HashMap<(u64, u64), u64> = HashMap::new();

        let part1: u64 = inputs
            .clone()
            .into_iter()
            .map(|num| blink(25, num, &mut cache))
            .sum();
        
        let part2: u64 = inputs
            .into_iter()
            .map(|num| blink(75, num, &mut cache))
            .sum();

        (part1, part2)
    }
    
}
