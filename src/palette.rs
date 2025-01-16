use regex::Regex;
use std::{fs, path::Path};
use termion::color;

pub fn read_palette(path: &Path) -> Vec<[u8; 3]> {
    println!("Reading palette {}", path.display());
    let content = match fs::read_to_string(path) {
        Ok(string) => string,
        Err(e) => panic!("Error reading palette: {}", e),
    };
    
    content
        .lines()
        .filter_map(|hex_code| parse_hex_str(hex_code))
        .collect()
}

fn parse_hex_str(str: &str) -> Option<[u8; 3]> {
    let pat = Regex::new("^#[0-9a-fA-F]{6}$").unwrap();
    if !pat.is_match(str) {
        return None;
    }
    
    let mut color: [u8; 3] = [0, 0, 0];
    for i in 0..3 {
        color[i] = u8::from_str_radix(&str[(i * 2 + 1)..(i * 2+3)], 16)
            .map_err(|_| "Invalid color value")
            .unwrap();
    }
    
    let term_color = color::Bg(color::Rgb(color[0], color[1], color[2]));
    println!(
        "{}  {} {str}", 
        term_color, 
        color::Bg(color::Reset),
    );
    
    Some(color)
}