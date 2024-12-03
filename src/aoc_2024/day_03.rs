use regex::Regex;

use crate::plugin::Plugin;
use crate::reader::Reader;

pub struct AoC2024Day03;

impl Plugin for AoC2024Day03 {
    fn execute(&self) -> (u64, u64) {
        let reader = Reader::new(3, 2024);
        let content = reader.load_puzzle();
        let mut enabled_op = true;
        let re = Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don\'t\(\))").unwrap();
        
        let res = re
            .captures_iter(content.as_str())
            .map(|cap| {
                match &cap[1] {
                    func if func.starts_with("mul") => {
                        if let (Some(x1), Some(x2)) = (cap.get(2), cap.get(3)) {
                            let x1: u64 = x1.as_str().parse().unwrap();
                            let x2: u64 = x2.as_str().parse().unwrap();

                            let part1 = x1 * x2;
                            let part2 = if enabled_op {
                                part1
                            } else {
                                0 as u64
                            };

                            (part1, part2)
                        } else {
                            (0 as u64, 0 as u64)
                        }
                    }
                    "do()" => {
                        enabled_op = true;
                        (0 as u64, 0 as u64)
                    }
                    "don't()" => {
                        enabled_op = false;
                        (0 as u64, 0 as u64)
                    }
                    _ => (0 as u64, 0 as u64)
                }
            })
            .fold((0, 0), |acc, (x, y)| (acc.0 + x, acc.1 + y));

        res
    }
}
