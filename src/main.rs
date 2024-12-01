mod config;
mod plugin;
mod reader;

mod aoc_2024;

use chrono::{Datelike, Utc};
use clap::{Arg, Command};

use plugin::PluginManager;

fn main() {
    let current_year = Utc::now().year() as u16;
    let matches = Command::new("aoc")
        .version("1.0")
        .author("Denis Selimovic <selimovicdenis98@gmail.com>")
        .about("Running AoC code")
        .arg(
            Arg::new("day")
                .short('d')
                .required(true)
                .value_parser(clap::value_parser!(u8))

        )
        .arg(
            Arg::new("year")
                .short('y')
                .required(false)
                .value_parser(clap::value_parser!(u16))
        )
        .get_matches();

    let day = matches.get_one::<u8>("day").unwrap();
    let year = match matches.get_one::<u16>("year") {
        Some(y) => y,
        None => &current_year,
    };

    let manager = PluginManager::new();
    let plugin = manager.get_plugin(*day, *year);
    let result = plugin.execute();
    
    println!("Part 1 solution {}; Part 2 solution {}", result.0, result.1);
}
