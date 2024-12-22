use std::boxed::Box;
use std::collections::HashMap;

use crate::aoc_2024::day_01;
use crate::aoc_2024::day_02;
use crate::aoc_2024::day_03;
use crate::aoc_2024::day_04;
use crate::aoc_2024::day_05;
use crate::aoc_2024::day_06;
use crate::aoc_2024::day_07;
use crate::aoc_2024::day_08;
use crate::aoc_2024::day_09;
use crate::aoc_2024::day_10;
use crate::aoc_2024::day_11;
use crate::aoc_2024::day_12;
use crate::aoc_2024::day_13;
use crate::aoc_2024::day_14;
use crate::aoc_2024::day_15;
use crate::aoc_2024::day_16;
use crate::aoc_2024::day_17;
use crate::aoc_2024::day_18;
use crate::aoc_2024::day_19;
use crate::aoc_2024::day_20;
use crate::aoc_2024::day_21;
use crate::aoc_2024::day_22;


pub trait Plugin {
    fn execute(&self) -> (u64, u64);
}

pub struct PluginManager {
    map: HashMap<(u8, u16), Box<dyn Plugin>>, 
}

impl PluginManager {
    pub fn new() -> Self {
        let mut map: HashMap<(u8, u16), Box<dyn Plugin>> = HashMap::new();
        map.insert((1 as u8, 2024 as u16), Box::new(day_01::AoC2024Day01));
        map.insert((2 as u8, 2024 as u16), Box::new(day_02::AoC2024Day02));
        map.insert((3 as u8, 2024 as u16), Box::new(day_03::AoC2024Day03));
        map.insert((4 as u8, 2024 as u16), Box::new(day_04::AoC2024Day04));
        map.insert((5 as u8, 2024 as u16), Box::new(day_05::AoC2024Day05));
        map.insert((6 as u8, 2024 as u16), Box::new(day_06::AoC2024Day06));
        map.insert((7 as u8, 2024 as u16), Box::new(day_07::AoC2024Day07));
        map.insert((8 as u8, 2024 as u16), Box::new(day_08::AoC2024Day08));
        map.insert((9 as u8, 2024 as u16), Box::new(day_09::AoC2024Day09));
        map.insert((10 as u8, 2024 as u16), Box::new(day_10::AoC2024Day10));
        map.insert((11 as u8, 2024 as u16), Box::new(day_11::AoC2024Day11));
        map.insert((12 as u8, 2024 as u16), Box::new(day_12::AoC2024Day12));
        map.insert((13 as u8, 2024 as u16), Box::new(day_13::AoC2024Day13));
        map.insert((14 as u8, 2024 as u16), Box::new(day_14::AoC2024Day14));
        map.insert((15 as u8, 2024 as u16), Box::new(day_15::AoC2024Day15));
        map.insert((16 as u8, 2024 as u16), Box::new(day_16::AoC2024Day16));
        map.insert((17 as u8, 2024 as u16), Box::new(day_17::AoC2024Day17));
        map.insert((18 as u8, 2024 as u16), Box::new(day_18::AoC2024Day18));
        map.insert((19 as u8, 2024 as u16), Box::new(day_19::AoC2024Day19));
        map.insert((20 as u8, 2024 as u16), Box::new(day_20::AoC2024Day20));
        map.insert((21 as u8, 2024 as u16), Box::new(day_21::AoC2024Day21));
        map.insert((22 as u8, 2024 as u16), Box::new(day_22::AoC2024Day22));

        Self { map }
    }

    pub fn get_plugin(&self, day: u8, year: u16) -> &Box<dyn Plugin> {
        match self.map.get(&(day, year)) {
            Some(res) => res,
            None => panic!("Missing module for day {}, year {}", day, year),
        }
    }
}
