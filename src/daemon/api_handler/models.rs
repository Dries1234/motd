use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config{
    pub key: String,
    pub city: String,
    pub message_type: String,
}

// reddit
#[derive(Debug,Serialize, Deserialize)]
#[serde(default)]
pub struct RedditPost{
   pub data: RedditPostData
}

#[derive(Debug,Serialize,Deserialize)]
#[serde(default)]
pub struct RedditPostData{
   pub title: String,
   pub selftext: String,
   pub author: String,
   pub stickied: bool,
   pub thumbnail: String,
   pub url: String,
}
#[derive(Debug,Serialize,Deserialize)]
#[serde(default)]
pub struct RedditHot{
    pub data: RedditData 
}
#[derive(Debug,Serialize,Deserialize)]
#[serde(default)]
pub struct RedditData{
    pub children: Vec<RedditPost>
}

impl Default for RedditPost{
    fn default() -> Self {
        Self { data: Default::default() }
    }
}
impl Default for RedditHot{
    fn default() -> Self {
        Self { data: Default::default() }
    }
}
impl Default for RedditData{
    fn default() -> Self {
        Self { children: Default::default() }
    }

}
impl Default for RedditPostData{
    fn default() -> Self {
        Self{title: Default::default(), selftext: Default::default(), author: Default::default(), stickied: Default::default(), thumbnail: Default::default(), url: Default::default()}
    }
}

// Weather
#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Weather{
    pub location: Location, 
    pub current: Current
}

#[derive(Debug,Serialize,Deserialize)]
#[serde(default)]
pub struct Location{
    pub name: String,
    pub country: String,
}
#[derive(Debug,Serialize,Deserialize)]
#[serde(default)]
pub struct Current{
    temp_c: f64,
    condition: Condition
}

#[derive(Debug,Serialize,Deserialize)]
#[serde(default)]
pub struct Condition{
    text: String
}

impl Default for Location{
    fn default() -> Self {
        Self{ name: Default::default(), country: Default::default()}
    }
}
impl Default for Weather{
    fn default() -> Self {
        Self{ location: Default::default(), current: Default::default()}
    }
}
impl Default for Current{
    fn default() -> Self {
        Self{temp_c: Default::default(), condition: Default::default()}
    }
}
impl Default for Condition{
    fn default() -> Self {
        Self{text: Default::default()}
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CombinedData{
    pub data: RedditData,
    pub location: Location, 
    pub current: Current      
}

impl Default for CombinedData{
    fn default() -> Self {
        Self { data: Default::default(), location: Default::default(), current: Default::default() }
    }
}