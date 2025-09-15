use serde::{Serialize, Deserialize};
use figlet_rs::FIGfont;
use colored::*;
use std::io::{self, Write};

#[derive(Serialize, Deserialize)]
struct Config {
    font: Option<String>,
    lines: Vec<String>,
    color: Option<(u8, u8, u8)>,
    center: Option<bool>, // <-- new field
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
    let config = Config::load();

    let standard_font = match config.font {
        Some(path) => FIGfont::from_file(&path.replace("~", &std::env::var("HOME").unwrap())).unwrap(),
        None => FIGfont::standard().unwrap(),
    };

    let term_width = termsize::get().map(|s| s.cols as usize).unwrap_or(80);
    let do_center = config.center.unwrap_or(false); // default false

    for line in config.lines {
        if let Some(figure) = standard_font.convert(&line) {
            let color = config.color.unwrap_or((255, 255, 255));
            
            for fig_line in figure.to_string().lines() {
                if do_center {
                    let line_length = fig_line.chars().count();
                    let padding = if term_width > line_length {
                        (term_width - line_length) / 2
                    } else {
                        0
                    };
                    print!("{:padding$}", "", padding = padding);
                }
                println!("{}", fig_line.truecolor(color.0, color.1, color.2));
            }
        }
    }
}
