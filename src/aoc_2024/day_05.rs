use std::collections::{HashMap, HashSet};
use std::cmp::Ordering;
use std::str::FromStr;
use itertools::Itertools;

use crate::plugin::Plugin;
use crate::reader::Reader;


pub struct AoC2024Day05;

fn part1(s: &str) -> i32 {
  evaluate_lines(s, evaluate_line_part1)
}

fn part2(s: &str) -> i32 {
  evaluate_lines(s, evaluate_line_part2)
}

fn evaluate_lines(s: &str, evaluate_line: fn(&HashMap<i32, HashSet<i32>>, &[i32]) -> Option<i32>) -> i32 {
  let (predecessors, lines) = parse(s);
  lines.iter().filter_map(|nums| evaluate_line(&predecessors, nums)).sum()
}

fn evaluate_line_part1(predecessors: &HashMap<i32, HashSet<i32>>, nums: &[i32]) -> Option<i32> {
  let cmp = |a, b| predecessors.get(b).cloned().unwrap_or_default().contains(a);
  nums.is_sorted_by(|a, b| cmp(a, b)).then_some(nums[nums.len() / 2])
}

fn evaluate_line_part2(predecessors: &HashMap<i32, HashSet<i32>>, nums: &[i32]) -> Option<i32> {
  let cmp = |a, b| predecessors.get(b).cloned().unwrap_or_default().contains(a);
  let to_ordering = |b| if b { Ordering::Less } else { Ordering::Greater };
  (!nums.is_sorted_by(|a, b| cmp(a, b))).then(|| {
    *nums.iter().sorted_by(|a, b| to_ordering(cmp(a, b))).collect_vec()[nums.len() / 2]
  })
}

fn parse(s: &str) -> (HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>) {
  let predecessors = s
    .lines()
    .take_while(|l| !l.is_empty())
    .fold(HashMap::<_, HashSet<_>>::new(), |mut m, line| {
      let (left, right) = line.split_once("|").unwrap();
      let (left, right) = (left.parse::<i32>().unwrap(), right.parse::<i32>().unwrap());
      m.entry(right).or_default().insert(left);
      m
    });

  let lines = s
    .lines()
    .skip_while(|line| !line.is_empty())
    .skip(1)
    .map(|line| {
      line
        .split(",")
        .map(i32::from_str)
        .map(Result::unwrap)
        .collect_vec()
    })
    .collect_vec();

  (predecessors, lines)
}


impl Plugin for AoC2024Day05 {
    fn execute(&self) -> (u64, u64) {
        let reader = Reader::new(5, 2024);
        let content = reader.load_puzzle();
        
        let part1: u64 = part1(&content) as u64;
        let part2: u64 = part2(&content) as u64;

        (part1, part2)
    }
}
