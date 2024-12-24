use core::panic;
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use regex::Regex;

use crate::plugin::Plugin;
use crate::reader::Reader;


pub struct AoC2024Day24;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
enum LogicOp {
    AND,
    OR,
    XOR,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Rule {
    v1: String,
    v2: String,
    op: LogicOp,
    res: String,
}

fn parse_input(content: &String) -> (HashMap<String, u8>, Vec<Rule>) {
    let re = Regex::new(r"([a-z0-9]{3}) (AND|OR|XOR) ([a-z0-9]{3}) -> ([a-z0-9]{3})").unwrap();
    let mut values = HashMap::new();
    let mut rules = Vec::new();

    for line in content.lines() {
        if line.contains(':') {
            let splt = line.split(": ").map(|s| s.to_string()).collect::<Vec<_>>();
            values.insert(splt[0].clone(), splt[1].parse::<u8>().unwrap());
        } else if line.contains("->") {
            if let Some(captures) = re.captures(line) {
                let v1 = captures.get(1).map_or("", |f| f.as_str()).to_string();
                let ops = captures.get(2).map_or("", |f| f.as_str());
                let v2 = captures.get(3).map_or("", |f| f.as_str()).to_string();
                let res = captures.get(4).map_or("", |f| f.as_str()).to_string();

                let op = match ops {
                    "AND" => LogicOp::AND,
                    "XOR" => LogicOp::XOR,
                    "OR" => LogicOp::OR,
                    _ => panic!("Unknown logic operator"),
                };

                rules.push(Rule { v1, v2, op, res });
            }
        }
    }

    (values, rules)
}

fn part1(values: &mut HashMap<String, u8>, rules: &Vec<Rule>) -> (u64, String) {
    let mut inbound: HashMap<Rule, usize> = HashMap::new();
    let mut outbound: HashMap<String, Vec<Rule>> = HashMap::new();

    for rule in rules {
        let mut cnt: usize = 0;

        if !values.contains_key(&rule.v1) {
            cnt += 1;
        }
        if !values.contains_key(&rule.v2) {
            cnt += 1;
        }

        inbound.insert(rule.clone(), cnt);
        outbound.entry(rule.v1.to_string()).or_insert_with(Vec::new).push(rule.clone());
        outbound.entry(rule.v2.to_string()).or_insert_with(Vec::new).push(rule.clone());
    }

    let mut todo = inbound
        .clone()
        .into_iter()
        .filter(|(_, v)| *v == 0)
        .map(|(k, _)| k)
        .collect::<Vec<_>>();

    while todo.len() > 0 {
        let mut nxt = Vec::new();

        for rule in todo {
            let v1 = values.get(&rule.v1).unwrap();
            let v2 = values.get(&rule.v2).unwrap();

            let res = match rule.op {
                LogicOp::OR => v1 | v2,
                LogicOp::AND => v1 & v2,
                LogicOp::XOR => v1 ^ v2,
            };
            values.insert(rule.res.clone(), res);

            if let Some(lst) = outbound.get(&rule.res) {
                for out in lst {
                    if let Some(val) = inbound.get(&out) {
                        if val - 1 == 0 {
                            inbound.remove(&out);
                            nxt.push(out.clone());
                        } else {
                            inbound.insert(out.clone(), val - 1);
                        }
                    }
                }
            }
        }

        todo = nxt;
    }

    let mut results = values
        .clone()
        .into_iter()
        .filter(|(k, _)| k.starts_with('z'))
        .map(|(k, _)| k)
        .collect::<Vec<_>>();
    results.sort();

    let final_res = results
        .clone()
        .into_iter()
        .rev()
        .map(|k| values.get(&k).unwrap())
        .fold(0, |acc, bit| (acc << 1) | *bit as u64);

    (final_res, results[results.len() - 1].clone())
}

fn does_not_start_with_any(s: &str, chars: &[char]) -> bool {
    match s.chars().next() {
        Some(first_char) => !chars.contains(&first_char),
        None => true, // An empty string does not start with any character
    }
}

fn part2(rules: &Vec<Rule>, high_z: String) -> HashSet<String> {
    let mut wrong = HashSet::new();

    for rule in rules {
        if rule.res.starts_with('z') && rule.op != LogicOp::XOR && rule.res != high_z {
            wrong.insert(rule.res.clone());
        }

        if rule.op == LogicOp::XOR
            && does_not_start_with_any(&rule.v1, &vec!['x', 'y', 'z']) 
            && does_not_start_with_any(&rule.v2, &vec!['x', 'y', 'z']) 
            && does_not_start_with_any(&rule.res, &vec!['x', 'y', 'z']) {
                wrong.insert(rule.res.clone());
        }

        if rule.op == LogicOp::AND && rule.v1 != "x00".to_string() && rule.v2 != "x00".to_string() {
            for sub_rule in rules.clone() {
                if (rule.res == sub_rule.v1 || rule.res == sub_rule.v2) && sub_rule.op != LogicOp::OR {
                    wrong.insert(rule.res.clone());
                }
            }
        }

        if rule.op == LogicOp::XOR {
            for sub_rule in rules.clone() {
                if (rule.res == sub_rule.v1 || rule.res == sub_rule.v2) && sub_rule.op == LogicOp::OR {
                    wrong.insert(rule.res.clone());
                }
            }
        }
    }

    let mut vec_wrong = wrong
        .clone()
        .into_iter()
        .collect::<Vec<_>>();
    vec_wrong.sort();
    let res_str = vec_wrong
        .into_iter()
        .join(",");
    
    println!("{}", res_str);

    wrong
}


impl Plugin for AoC2024Day24 {
   fn execute(&self) -> (u64, u64) {
        let reader = Reader::new(24, 2024);
        let puzzle = reader.load_puzzle();
        let (mut values, rules) = parse_input(&puzzle);

        let (p1, high_z) = part1(&mut values, &rules);
        let p2 = part2(&rules, high_z);


        (p1, p2.len() as u64)
   } 
}
