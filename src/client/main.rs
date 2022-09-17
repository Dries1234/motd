use models::{Config, Logger, MessageType, RedditHot, Weather};
use serde::de::value::Error;
use std::{io::Write, path::PathBuf};

const CONFIG: &str = "motd.json";

mod models;
fn main() {
    let config = load_config();
    let mut path = dirs::cache_dir().unwrap();
    path.push("motd.json");
    let file = std::fs::File::open(path).expect("Could not open file!");
    match config {
        Ok(conf) => match MessageType::try_from(conf.message_type.as_str()) {
            Ok(message_type) => match message_type {
                MessageType::WEATHER => {
                    let weather: Weather =
                        serde_json::from_reader(file).expect("Could not read from file");
                    weather.display();
                }
                MessageType::REDDIT => {
                    let reddit: RedditHot =
                        serde_json::from_reader(file).expect("Could not read from file");
                    reddit.display();
                }
            },
            Err(_) => (),
        },
        Err(_) => (),
    }
}
fn load_config() -> Result<Config, Error> {
    let config_dir = dirs::config_dir();
    let config_file: PathBuf;
    let config_json: Config;
    match config_dir {
        Some(mut config) => {
            config.push(CONFIG);
            config_file = config;
        }
        None => {
            config_file = PathBuf::from("~/.config/motd.json");
        }
        
    }
    let b = std::path::Path::new(&config_file).exists();
    if b {
        let config_text = std::fs::read_to_string(config_file).unwrap();
        config_json = serde_json::from_str(&config_text).unwrap();
    } else {
        let mut new = std::fs::File::create(config_file).expect("Could not create config file!");
        let default_text: &str = include_str!("config_example.json");
        config_json = serde_json::from_str(&default_text).unwrap();
        new.write_all(&default_text.as_bytes())
            .expect("Failed to create default config!");
    }

    Ok(config_json)
}
