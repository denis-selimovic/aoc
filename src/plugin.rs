use std::boxed::Box;
use std::collections::HashMap;

use crate::aoc_2024::day_01;
use crate::aoc_2024::day_02;
use crate::aoc_2024::day_03;
use crate::aoc_2024::day_04;
use crate::aoc_2024::day_05;


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

        Self { map }
    }

    pub fn get_plugin(&self, day: u8, year: u16) -> &Box<dyn Plugin> {
        match self.map.get(&(day, year)) {
            Some(res) => res,
            None => panic!("Missing module for day {}, year {}", day, year),
        }
    }
}
