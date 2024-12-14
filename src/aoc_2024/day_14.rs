use itertools::Itertools;
use regex::Regex;

use crate::plugin::Plugin;
use crate::reader::Reader;

pub struct AoC2024Day14;

fn positive_mod(x: i32, y: i32) -> i32 {
    let mut r = x % y;
    r = r + y;

    return r % y;
}

#[derive(Clone, Debug)]
struct Robot {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

impl Robot {
    fn rmove(&self, m: i32, n: i32, sec: i32) -> Self {
        let mut new_x = self.x + self.vx * sec;
        let mut new_y = self.y + self.vy * sec;
        
        new_x = positive_mod(new_x, m);
        new_y = positive_mod(new_y, n);

        assert!(new_x >= 0 && new_x < m);
        assert!(new_y >= 0 && new_y < n);

        Robot { x: new_x, y: new_y, vx: self.vx, vy: self.vy }
    }
}

fn parse_line(line: &String, reg: &Regex) -> Vec<i32> {
    reg
        .captures_iter(line)
        .filter_map(|cap| cap.get(1).map(|m| m.as_str().parse::<i32>().unwrap()))
        .collect()
}

fn parse(content: &String) -> Vec<Robot> {
    let regex = Regex::new(r"(-?\d+)").unwrap();

    content
        .lines()
        .map(|l| parse_line(&l.to_string(), &regex))
        .map(|v| Robot { x: v[0], y: v[1], vx: v[2], vy: v[3] } )
        .collect()
}

fn quadrant(robot: &Robot, m: i32, n: i32) -> Option<i32> {
    let w = m / 2;
    let h = n / 2;

    match *robot {
        Robot { x, y, vx: _, vy: _ } if x < w && y < h => Some(0),
        Robot { x, y, vx: _, vy: _ } if x < w && y > h => Some(1),
        Robot { x, y, vx: _, vy: _ } if x > w && y < h => Some(2),
        Robot { x, y, vx: _, vy: _ } if x > w && y > h => Some(3),
        _ => None,
    }
}

fn quadrant_count_product(robots: &Vec<Robot>, m: i32, n: i32, seconds: i32) -> usize {
    robots
        .into_iter()
        .map(|r| r.rmove(m, n, seconds))
        .filter_map(|r| quadrant(&r, m, n))
        .counts()
        .values()
        .product()
}

impl Plugin for AoC2024Day14 {
    fn execute(&self) -> (u64, u64) {
        let reader = Reader::new(14, 2024);
        let content = reader.load_puzzle();

        let robots = parse(&content);
        let (m, n) = (101, 103);

        let part1 = quadrant_count_product(&robots, m, n, 100);
        let part2 = (1..=101*103)
            .map(|t| (t, quadrant_count_product(&robots, m, n, t)))
            .min_by(|&x, &y| x.1.cmp(&y.1))
            .unwrap()
            .0;

        (part1 as u64, part2 as u64)
    }
}
