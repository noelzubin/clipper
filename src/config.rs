extern crate dirs;

use std::fs;
use std::process;
use toml::value::Table;
use toml::Value;

fn parse_config_toml(conf_str: &str) -> Config {
    let conf: Table = conf_str.parse::<Value>().unwrap().try_into().unwrap();
    Config(conf)
}

#[derive(Debug, PartialEq)]
pub struct Config(Table);

impl Config {
    pub fn new() -> Self {
        let mut config_file_path = dirs::home_dir().unwrap();
        config_file_path.push(".clipper.conf");
        match fs::read_to_string(config_file_path) {
            Ok(contents) => parse_config_toml(&contents),
            Err(e) => {
                eprintln!("{}", e.to_string());
                process::exit(1);
            }
        }
    }

    pub fn get_config(&self) -> &Table {
        &self.0
    }
}