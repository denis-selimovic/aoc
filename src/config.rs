use std::env;
use dotenv::dotenv;


fn load_env_variable(variable_name: &String, default: Option<String>) -> String {
    match env::var(variable_name) {
        Ok(value) => value,
        Err(_) => {
            match default {
                Some(def) => def,
                None => panic!("Unable to load env variable: {}\n", variable_name), 
            }
        }
    }
}

pub struct Config {
    pub base_url: String,
    pub session: String,
}

impl Config {
    pub fn new() -> Self {
        match dotenv() {
            Ok(_) => print!("Successfully loaded environment\n"),
            Err(_) => panic!("Unable to load environment\n"),
        }


        Self {
            base_url: load_env_variable(&"BASE_URL".to_string(), None),
            session: load_env_variable(&"SESSION".to_string(), None),
        }
    }
}