use std::usize;

use crate::plugin::Plugin;
use crate::reader::Reader;

pub struct AoC2024Day09;

#[derive(Clone, Debug)]
struct FileBlock {
    file_id: usize,
    is_free: bool,
}


fn make_layout(content: &String) -> Vec<FileBlock> {
    let mut res = Vec::new();
    let blocks: Vec<usize> = content.chars().map(|c| c.to_digit(10).unwrap() as usize).collect();
    
    let mut id = 0;
    let mut is_file = true;

    for ch in blocks {

        if is_file {
            for _ in 0..ch {
                res.push(FileBlock{ file_id: id, is_free: false });
            }

            id += 1;
        } else {
            for _ in 0..ch {
                res.push(FileBlock{ file_id: 0, is_free: true });
            }
        }

        is_file = !is_file;
    }

   res
}


impl Plugin for AoC2024Day09 {
    fn execute(&self) -> (u64, u64) {
        let reader = Reader::new(9, 2024);
        let puzzle = reader.load_puzzle().trim_end().to_string();
        
        let mut file_system = make_layout(&puzzle);
        let (mut left, mut right) = (0, file_system.len() - 1);

        while left < right {
            while !file_system[left].is_free {
                left += 1;
            }
            while file_system[right].is_free {
                right -= 1;
            }

            if left < right {
                file_system.swap(left, right);
            }

            left += 1;
            right -= 1;
        }

        let part1: u64 = file_system
            .clone()
            .into_iter()
            .enumerate()
            .map(|(i, b)| (i * b.file_id) as u64)
            .sum();


        file_system = make_layout(&puzzle);
        right = file_system.len() - 1;
        let mut id: usize = file_system.clone().into_iter().map(|b| b.file_id).max().unwrap();

        while right > 0 {
            let mut n = right as usize;

            while file_system[n].is_free || (!file_system[n].is_free && file_system[n].file_id != id) {
                n -= 1;
            }

            let mut last = 0;
            let mut left_cache = 0;
            let mut right_cache = 0;

            while n >= right_cache && !file_system[n - right_cache].is_free && file_system[n - right_cache].file_id == id {
                right_cache += 1;
            }

            if id == 0 {
                break;
            }

            id -= 1;

            while left_cache < right_cache && last <= n {
                last += left_cache;
                left_cache = 0;

                while !file_system[last].is_free {
                    last += 1;
                }

                while last + left_cache <= n && file_system[last + left_cache].is_free {
                    left_cache += 1;
                }
            }

            if left_cache >= right_cache {
                for l in 0..right_cache {
                    file_system.swap(last + l, n - l);
                }
            }

            right = (n - right_cache) as usize;
        }

        let part2: u64 = file_system
            .clone()
            .into_iter()
            .enumerate()
            .map(|(i, b)| (i * b.file_id) as u64)
            .sum();

        (part1, part2)
    }
    
}
