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
        let url = &format!("{}/{}/day/{}/input", config.base_url, self.year, self.day);

        let client = Client::new();
        let cookie = format!("session={}", config.session);
        
        match client.get(url).header(COOKIE, cookie).send() {
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
        }
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
