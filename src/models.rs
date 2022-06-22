use serde::{Deserialize, Serialize};
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
        Self{title: Default::default(), selftext: Default::default(), author: Default::default(), url: Default::default()}
    }
}