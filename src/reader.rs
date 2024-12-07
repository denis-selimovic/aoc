use std::fs;

use reqwest::blocking::Client;
use reqwest::header::COOKIE;
use reqwest::StatusCode;

use crate::config::Config;

pub struct Reader{
    day: u8,
    year: u16,
}

impl Reader {
    pub fn new(day: u8, year: u16) -> Self {
        Self { day, year }
    }

    pub fn load_puzzle(&self) -> String {
        let config = Config::new();
        let cache_dir = config.cache_dir.join(self.year.to_string());
        let cache_path = cache_dir.join(format!("{}.txt", self.day));
        
        if cache_path.exists() && cache_path.is_file() {
            match fs::read_to_string(&cache_path) {
                Ok(content) => {
                    println!("Returning data from cache {}", cache_path.to_str().unwrap());

                    return content;
                },
                Err(err) => println!("Error {} while reading cached version of puzzle in file {}", err, cache_path.to_str().unwrap()),
            }
        }

        let url = &format!("{}/{}/day/{}/input", config.base_url, self.year, self.day);
        let client = Client::new();
        let cookie = format!("session={}", config.session);
        
        let content = match client.get(url).header(COOKIE, cookie).send() {
            Ok(resp) => {
                match resp.status() {
                    StatusCode::OK => {
                        match resp.text() {
                            Ok(text) => text,
                            Err(err) => panic!("Error while reading data from url {} with message {}", url, err),
                        }
                    },
                    _ => panic!("Loading url {} ended with HTTP code {}", url, resp.status())
                }
            },
            Err(err) => panic!("Unable to load puzzle with url {}. Error {} occured!\n", url, err),
        };

        match fs::create_dir_all(&cache_dir) {
            Ok(_) => println!("Sucessfully crated cache dir {}", cache_dir.to_str().unwrap()),
            Err(err) => panic!("Couldn't create cache dir {} with err {}", cache_dir.to_str().unwrap(), err),
        }

        match fs::write(&cache_path, &content) {
            Ok(_) => println!("Succesfully created cache file {}", cache_path.to_str().unwrap()),
            Err(err) => panic!("Couldn't create cache file {} with err {}", cache_path.to_str().unwrap(), err),
        }

        content
    }

    pub fn to_grid(&self, content: &String) -> Vec<Vec<String>> {
        let mut result = Vec::new();

        for line in content.lines() {
            let mut row = Vec::new();

            for ch in line.chars() {
                row.push(ch.to_string());
            }

            result.push(row);
        }

        result
    }

    pub fn split(&self, content: &String, delimiter: &str) -> Vec<Vec<String>> {
        let mut result = Vec::new();

        for line in content.lines() {
            let mut row = Vec::new();
            let parts: Vec<&str> = line.split(delimiter).collect();

            for part in parts.iter() {
                row.push(part.to_string());
            }

            result.push(row);
        }

        result
    }

    pub fn split_vertically(&self, content: &String, delimiter: &str, total: usize) -> Vec<Vec<String>> {
        let mut result = Vec::with_capacity(total);

        for _ in 0..total {
            result.push(Vec::new());
        }

        for line in content.lines() {
            let parts: Vec<&str> = line.split(delimiter).collect();
            assert_eq!(parts.len(), total);

            for (i, part) in parts.iter().enumerate() {
                result[i].push(part.to_string());
            }
        }

        result 
    }
}
