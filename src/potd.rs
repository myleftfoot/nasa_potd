use serde_derive::Deserialize;
use std::fs::File;
use std::io::prelude::*;

#[derive(Deserialize)]
pub struct Config {
    pub destination_folder: String,
    pub convert_to_png: bool,
    pub nasa: Nasa,
}

#[derive(Deserialize)]
pub struct Nasa {
    pub api_key: String,
}

pub fn parse_config(config_file: String) -> Config {
    let mut file_content = String::new();
    File::open(&config_file)
        .and_then(|mut f| f.read_to_string(&mut file_content))
        .unwrap();

    toml::from_str(&mut file_content).unwrap()
}
