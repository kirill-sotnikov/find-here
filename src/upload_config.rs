use std::{fs::File, io::Read, process};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ConfigColor {
    Blue,
    Green,
    Red,
    None,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub background: bool,
    pub underline: bool,
    pub color: ConfigColor,
}

pub fn upload_config() -> Config {
    let mut file = match File::open("/etc/fh-config/config.json") {
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => {
                return Config {
                    background: false,
                    underline: true,
                    color: ConfigColor::Green,
                }
            }
            _ => {
                println!("{}", error);
                process::exit(1);
            }
        },
        Ok(data) => data,
    };

    let mut row_config = String::new();

    file.read_to_string(&mut row_config)
        .unwrap_or_else(|error| match error.kind() {
            std::io::ErrorKind::PermissionDenied => {
                println!("Permission denied!");
                process::exit(1);
            }
            _ => {
                println!("{}", error);
                process::exit(1);
            }
        });

    let config_from_file: Config = match serde_json::from_str(&row_config) {
        Err(error) => {
            println!("Error config");
            println!("{}", error);
            process::exit(1);
        }
        Ok(data) => data,
    };

    config_from_file
}
