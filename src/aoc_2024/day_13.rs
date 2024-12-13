use regex::Regex;

use crate::plugin::Plugin;
use crate::reader::Reader;

pub struct AoC2024Day13;

#[derive(Clone, Debug)]
struct Equation {
    a: i64,
    b: i64,
    c: i64,
}

#[derive(Clone, Debug)]
struct EquationSystem {
    first: Equation,
    second: Equation,
}


impl EquationSystem {
    fn solve(&self) -> Option<(i64, i64)> {
        let det = self.first.a * self.second.b - self.first.b * self.second.a;

        if det == 0 {
            return None;
        }

        let det_x = self.first.c * self.second.b - self.first.b * self.second.c;
        let det_y = self.first.a * self.second.c - self.first.c * self.second.a;

        if det_x % det != 0 || det_y % det != 0 {
            return None;
        }

        Some((det_x / det, det_y / det))
    }
}

fn parse(content: &String) -> (Vec<EquationSystem>, Vec<EquationSystem>) {
    let mut res = Vec::new();
    let mut res2 = Vec::new();
    let mut line_iter = content.lines();

    let button_re = Regex::new(r"[XY]\+(\d+)").unwrap();
    let prize_re = Regex::new(r"[XY]\=(\d+)").unwrap();

    while let (Some(first), Some(second), Some(third)) = (line_iter.next(), line_iter.next(), line_iter.next()) {
        let p1: Vec<i64> = button_re
            .captures_iter(first)
            .filter_map(|cap| cap.get(1).map(|m| m.as_str().parse::<i64>().unwrap()))
            .collect();
        let p2: Vec<i64> = button_re
            .captures_iter(second)
            .filter_map(|cap| cap.get(1).map(|m| m.as_str().parse::<i64>().unwrap()))
            .collect();
        let b: Vec<i64> = prize_re
            .captures_iter(third)
            .filter_map(|cap| cap.get(1).map(|m| m.as_str().parse::<i64>().unwrap()))
            .collect();

        let first = Equation { a: p1[0], b: p2[0], c: b[0] };
        let second = Equation { a: p1[1], b: p2[1], c: b[1] };
        res.push(EquationSystem{ first, second });

        let third = Equation { a: p1[0], b: p2[0], c: b[0] + 10000000000000 };
        let fourth = Equation { a: p1[1], b: p2[1], c: b[1] + 10000000000000 };
        res2.push(EquationSystem{ first: third, second: fourth });

        let _ = line_iter.next();
    }

    (res, res2) 
}


impl Plugin for AoC2024Day13 {
    fn execute(&self) -> (u64, u64) {
        let reader = Reader::new(13, 2024);
        let content = reader.load_puzzle();
        let (s1, s2) = parse(&content);

        let part1: u64 = s1
            .into_iter()
            .filter_map(|s| s.solve())
            .map(|(a, b)| (3 * a + b) as u64)
            .sum();

        let part2: u64 = s2
            .into_iter()
            .filter_map(|s| s.solve())
            .map(|(a, b)| (3 * a + b) as u64)
            .sum();

        (part1, part2)
    }
}
