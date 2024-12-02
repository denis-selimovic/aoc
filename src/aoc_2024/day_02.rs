use crate::plugin::Plugin;
use crate::reader::Reader;

pub struct AoC2024Day02;


fn is_good(seq: &Vec<i32>) -> bool {
    let gaps: Vec<i32> = seq
        .clone()
        .into_iter()
        .enumerate()
        .skip(1)
        .map(|(idx, _)| seq[idx] - seq[idx - 1])
        .collect();

    gaps.iter().all(|&x| x > 0 && x < 4) || gaps.iter().all(|&x| x > -4 && x < 0) 
}


impl Plugin for AoC2024Day02 {
    fn execute(&self) -> (u64, u64) {
        let reader = Reader::new(2, 2024);
        let puzzle = reader.load_puzzle();
        let input = reader.split(&puzzle, " ");
        
        let parsed: Vec<Vec<i32>> = input
            .into_iter()
            .map(|inner| {
                inner
                    .into_iter()
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<Vec<i32>>>();

        let part1 = parsed
            .clone()
            .into_iter()
            .map(|inner| {
                if is_good(&inner) {
                    return 1;
                } else {
                    return 0;
                }
            })
            .sum();
        let part2 = parsed
            .into_iter()
            .map(|inner| {
                if is_good(&inner) {
                    return 1;
                }

                let results: Vec<Vec<i32>> = (0..inner.len())
                    .map(|i| {
                        inner[0..i].iter()
                        .chain(&inner[i + 1..])
                        .copied() // Copy values into a new Vec
                        .collect()
                    })
                    .collect();

                if results.iter().any(|vec| is_good(vec)) {
                    return 1;
                } else {
                    return 0;
                }
            })
            .sum();

        (part1, part2)
    }
}
