use colored::{Color, Colorize};
use rand::Rng;
use serde::{Deserialize, Serialize};
const GREY: Color = Color::TrueColor {
    r: (119),
    g: (118),
    b: (119),
};

// gruvbox colors
const GREEN: Color = Color::TrueColor {
    r: (142),
    g: (192),
    b: (124),
};
const BLUE: Color = Color::TrueColor {
    r: (131),
    g: (165),
    b: (152),
};
const DARK_BLUE: Color = Color::TrueColor {
    r: (69),
    g: (133),
    b: (136),
};

const ZERO: [&str; 3] = [" __ ", "|  |", "|__|"];
const ONE: [&str; 3] = ["    ", "   |", "   |"];
const TWO: [&str; 3] = [" __ ", " __|", "|__ "];
const THREE: [&str; 3] = [" __ ", " __|", " __|"];
const FOUR: [&str; 3] = ["    ", "|__|", "   |"];
const FIVE: [&str; 3] = [" __ ", "|__ ", " __|"];
const SIX: [&str; 3] = [" __ ", "|__ ", "|__|"];
const SEVEN: [&str; 3] = [" __", "   |", "   |"];
const EIGHT: [&str; 3] = [" __ ", "|__|", "|__|"];
const NINE: [&str; 3] = [" __ ", "|__|", " __|"];
const DIVIDER: [&str; 3] = ["   ", " : ", "   "];

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub key: String,
    pub city: String,
    pub message_type: String,
}
pub enum MessageType {
    REDDIT,
    WEATHER,
}

impl TryFrom<&str> for MessageType {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "reddit" => Ok(MessageType::REDDIT),
            "weather" => Ok(MessageType::WEATHER),
            &_ => Err(()),
        }
    }
}
// reddit
#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct RedditPost {
    pub data: RedditPostData,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct RedditPostData {
    pub title: String,
    pub selftext: String,
    pub author: String,
    pub stickied: bool,
    pub thumbnail: String,
    pub url: String,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct RedditHot {
    pub data: RedditData,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct RedditData {
    pub children: Vec<RedditPost>,
}

impl Default for RedditPost {
    fn default() -> Self {
        Self {
            data: Default::default(),
        }
    }
}
impl Default for RedditHot {
    fn default() -> Self {
        Self {
            data: Default::default(),
        }
    }
}
impl Default for RedditData {
    fn default() -> Self {
        Self {
            children: Default::default(),
        }
    }
}
impl Default for RedditPostData {
    fn default() -> Self {
        Self {
            title: Default::default(),
            selftext: Default::default(),
            author: Default::default(),
            stickied: Default::default(),
            thumbnail: Default::default(),
            url: Default::default(),
        }
    }
}

// Weather
#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Weather {
    pub location: Location,
    pub current: Current,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Location {
    pub name: String,
    pub country: String,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Current {
    temp_c: f32,
    condition: Condition,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Condition {
    text: String,
}

impl Default for Location {
    fn default() -> Self {
        Self {
            name: Default::default(),
            country: Default::default(),
        }
    }
}
impl Default for Weather {
    fn default() -> Self {
        Self {
            location: Default::default(),
            current: Default::default(),
        }
    }
}
impl Default for Current {
    fn default() -> Self {
        Self {
            temp_c: Default::default(),
            condition: Default::default(),
        }
    }
}
impl Default for Condition {
    fn default() -> Self {
        Self {
            text: Default::default(),
        }
    }
}

impl Logger for Weather {
    fn display(&self) {
        // define digit characters

        // get the current time
        let time = chrono::Local::now();
        let time_string = time.format("%H:%M").to_string();
        let mut digits: Vec<u32> = vec![];
        for digit in time_string.chars() {
            if digit == ':' {
                digits.push(99);
            }
            match digit.to_digit(10) {
                Some(i) => {
                    digits.push(i);
                }
                None => (),
            }
        }
        // build the clock string
        let mut clock = String::new();
        for d in 0..3 {
            for digit in &digits {
                match digit {
                    0 => clock.push_str(ZERO[d]),
                    1 => clock.push_str(ONE[d]),
                    2 => clock.push_str(TWO[d]),
                    3 => clock.push_str(THREE[d]),
                    4 => clock.push_str(FOUR[d]),
                    5 => clock.push_str(FIVE[d]),
                    6 => clock.push_str(SIX[d]),
                    7 => clock.push_str(SEVEN[d]),
                    8 => clock.push_str(EIGHT[d]),
                    9 => clock.push_str(NINE[d]),
                    99 => clock.push_str(DIVIDER[d]),
                    _ => (),
                }
            }
            clock.push_str("\n")
        }
        let rows = clock.split("\n");
        // print the clock
        for row in rows {
            println!("{}", row.color(GREEN));
        }
        println!(
            "{} {}",
            self.location.name.color(BLUE).bold(),
            self.location.country.color(BLUE).bold()
        );
        // temperature
        println!(
            "{} {}",
            temperature_format(self.current.temp_c),
            self.current.condition.text.color(DARK_BLUE)
        );
    }
}

fn temperature_format(value: f32) -> String {
    let mut res: String;
    if value < 10.0 {
        res = value.to_string();
        res.push_str("°C");
        res = res.bright_blue().to_string();
    } else if value > 21.0 {
        res = value.to_string();
        res.push_str("°C");
        res = res.red().to_string();
    } else {
        res = value.to_string();
        res.push_str("°C");
        res = res.bright_green().to_string();
    }
    res
}

impl Logger for RedditHot {
    fn display(&self) {
        let posts = &self.data.children;
        let random_post = &posts[rand::thread_rng().gen_range(0..posts.len())].data;
        // Print the title
        let title = "Joke of the day:";
        println!("{}\n", title.bold().red());

        // Print the joke
        // title
        let selftext = random_post.selftext.replace("&amp;#x200B;", "");
        println!("*{}*\n", random_post.title);
        // the body
        println!("{}\n", selftext);
        println!("Posted by: {}\n\n", random_post.author.color(GREY).dimmed());
    }
}

pub trait Logger {
    fn display(&self);
}
