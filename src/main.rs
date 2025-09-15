use serde::{Serialize, Deserialize};
use figlet_rs::FIGfont;
use colored::*;

#[derive(Serialize, Deserialize)]
struct Config {
    font: Option<String>,
    lines: Vec<String>,
    color: Option<(u8, u8, u8)>
}

impl Config {
    fn load() -> Self {
        let home_dir = std::env::var("HOME").unwrap();
        let config_file = format!("{}/.config/welcome-msg/config.json", home_dir);
        let config = std::fs::read_to_string(config_file).unwrap();
        serde_json::from_str(&config).unwrap()
    }
}

fn main() {
    // Load the standard font
    let config = Config::load();

    let standard_font = match config.font {
        Some(a) => FIGfont::from_file(&a.replace("~", &std::env::var("HOME").unwrap())).unwrap(),
        None => FIGfont::standard().unwrap(),
    };
    
    for line in config.lines {
        let figure = standard_font.convert(&line).unwrap();
        let color = config.color.unwrap_or((0, 0, 0));
        println!("{}", figure.to_string().color(Color::TrueColor { r: color.0 , g: color.1, b: color.2 }));
    }
}
