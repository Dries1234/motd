use std::{io::{Write, Error}, path::PathBuf};

use api_handler::{
    get,
    models::{CombinedData, Config, RedditHot, RedditPost, Weather},
};
use colored::Colorize;

mod api_handler;

fn main() {
    match load_config(){
        Ok(config) => fetch_data(config),
        Err(_) => ()
    }
}

fn fetch_posts() -> Result<RedditHot, Box<(dyn std::error::Error + 'static)>> {
    let hotpage: Result<RedditHot, _> = api_handler::get("https://reddit.com/r/jokes/hot.json");
    return hotpage;
}

fn fetch_weather(config: Config) -> Result<Weather, Box<(dyn std::error::Error + 'static)>> {
    if config.key.is_empty(){
        println!("{} {}", "Please provide a weather api key :".bold().red(), "(https://www.weatherapi.com/)".bold().blue())
    }
    let weather: Result<Weather, _> = get(format!(
        "http://api.weatherapi.com/v1/current.json?key={}&q={}",
        config.key,
        config.city
    ));
    return weather;
}

fn fetch_data(config: Config) {
    let weather = fetch_weather(config);
    let reddit_hot = fetch_posts();
    let mut combined = CombinedData{
        data: Default::default(),
        location: Default::default(),
        current: Default::default(),
    };
    match reddit_hot {
        Ok(mut hot) => {
            let filtered: Vec<RedditPost> = hot
                .data
                .children
                .into_iter()
                .filter(|post| (post.data.stickied == false && post.data.thumbnail.is_empty()))
                .collect();
            hot.data.children = filtered;
            combined.data = hot.data;
        }
        Err(_) => (),
    }

    match weather {
        Ok(w) => {
            combined.current = w.current;
            combined.location = w.location;
        }
        Err(_) => (),
    }
    let mut dir = dirs::cache_dir().unwrap();
    dir.push("motd.json");
    let mut file = std::fs::File::create(dir).expect("File creation failed");
    file.write_all(
        serde_json::to_string_pretty(&combined)
            .unwrap()
            .as_bytes(),
    )
    .unwrap();
}

fn load_config() -> Result<Config, Error> {
    let config_dir = dirs::config_dir();
    let config_file: PathBuf;
    let config_json: Config;
    match config_dir {
        Some(mut config) => {
            config.push("motd.json");
            config_file = config;
        }
        None => {
            config_file = PathBuf::from("~/.config/motd.json");
        }
    }
    let b = std::path::Path::new(&config_file).exists();
    if b {
        let config_text = std::fs::read_to_string(config_file).unwrap();
        config_json = serde_json::from_str(&config_text)?;
    } else {
        let mut new = std::fs::File::create(config_file).expect("Could not create config file!");
        let default_text = include_str!("config_example.json");
        config_json = serde_json::from_str(&default_text).unwrap();
        new.write_all(&default_text.as_bytes())
            .expect("Failed to create default config!");
    }

    Ok(config_json)
}
