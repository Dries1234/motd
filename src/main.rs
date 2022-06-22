use colored::{Colorize, Color};
use models::{RedditHot, RedditPostData};
use rand::Rng;

const GREY: Color = Color::TrueColor { r: (119), g: (118), b: (119) };
const CONFIG: &str = "motd.json";

mod models;
fn main() {    
    let mut path = dirs::cache_dir().unwrap();
    path.push(CONFIG);
    let file = std::fs::File::open(path).expect("Could not open file!");
    let data: RedditHot = serde_json::from_reader(file).expect("Could not read file!");
    let posts = data.data.children;
    let random_post = &posts[rand::thread_rng().gen_range(0..posts.len())].data;
    print_motd(random_post);
}

fn print_motd(post : &RedditPostData){
    // Print the title
    let title = "Joke of the day:";
    println!("{}\n", title.bold().red());

    // Print the joke
    //title
    let selftext = post.selftext.replace("&amp;#x200B;", "");
    println!("*{}*\n", post.title);
    println!("{}\n", selftext);
    println!("Posted by: {}\n\n", post.author.color(GREY).dimmed());
}