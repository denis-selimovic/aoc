use crate::plugin::Plugin;
use crate::reader::Reader;

pub struct AoC2024Day07;

#[derive(Clone, Debug)]
struct Entry {
    result: u64,
    operands: Vec<u64>,
}


fn load_input(content: &String) -> Vec<Entry> {
    content
        .lines()
        .into_iter()
        .map(|l| {
            let mut parts = l.splitn(2, ": ");
            let result = parts.next().unwrap();
            let operands = parts.next().unwrap();

            Entry {
                result: result.parse::<u64>().unwrap(),
                operands: operands
                    .split(" ")
                    .map(|i| i.parse::<u64>().unwrap())
                    .collect()
            }
            
        })
        .collect()
}

fn is_valid(result: u64, partial: u64, values: &Vec<u64>, idx: usize) -> bool {
    if idx == values.len() {
        return result == partial;
    }

    is_valid(result, partial + values[idx], values, idx + 1) || is_valid(result, partial * values[idx], values, idx + 1)
}

fn is_valid_2(result: u64, partial: u64, values: &Vec<u64>, idx: usize) -> bool {
    if idx == values.len() {
        return result == partial;
    }

    if partial > result {
        return false;
    }

    let add = partial + values[idx];
    let mult = partial * values[idx];
    let concat = format!("{}{}", partial, values[idx]).parse::<u64>().unwrap();

    is_valid_2(result, add, values, idx + 1) || is_valid_2(result, mult, values, idx + 1) || is_valid_2(result, concat, values, idx + 1)
}


impl Plugin for AoC2024Day07 {
    fn execute(&self) -> (u64, u64) {
        let reader = Reader::new(7, 2024);
        let content = reader.load_puzzle();
        let entries = load_input(&content);

        let part1 = entries
            .clone()
            .into_iter()
            .filter(|e| is_valid(e.result, e.operands[0], &e.operands, 1))
            .map(|e| e.result)
            .sum();

        let part2 = entries
            .into_iter()
            .filter(|e| is_valid_2(e.result, e.operands[0], &e.operands, 1))
            .map(|e| e.result)
            .sum();

        (part1, part2)
    }
}
