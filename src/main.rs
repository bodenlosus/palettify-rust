extern crate image;
extern crate regex;

use regex::Regex;
use image::{ImageBuffer, Rgb, ImageReader};
use std::{fs};

fn main() {
    // Create an 800x600 image with black pixels
    let dyn_img = match ImageReader::open("./RGB_24bits_palette_R255.png") {
        Ok(r) => r,
        Err(e) => panic!("Error opening image: {}", e),
    };
    let mut img = dyn_img.decode().unwrap().to_rgb8();

    // Set each pixel to a color
    for (x, y, pixel) in img.enumerate_pixels_mut() {          // Blue channel
        let r = (pixel[0] / 2) as u8;
        let g = (pixel[1] / 2) as u8;
        let b = (pixel[2] / 2) as u8;
        *pixel = Rgb([r, g, b]);
    }

    // Save the image to a file
    img.save("output.png").unwrap();
    let palette = read_palette("/home/johannes/Pictures/horizon_theme");
}

fn interpolate(color: [u8; 3], palette:Vec<[u8; 3]>){
    let mut dists: Vec<f32> = Vec::new();
    for pcolor in palette.iter() {
        let dr = pcolor[0] - color[0];
        let dg = pcolor[1] - color[1];
        let db = pcolor[2] - color[2];
        let ds = ((dr * dr + dg * dg + db * db)) as f32;
        dists.push(ds.sqrt());   
    }
}

fn read_palette(path: &str) -> Vec<[u8; 3]>{
    println!("Reading palette {path}");
    let content = match fs::read_to_string(path) {
        Ok(string) => string,
        Err(e) => panic!("Error reading palette: {}", e),
    };
    let palette: Vec<[u8; 3]> = content
        .split("\n")
        .map(|hex_code| parse_hex_str(hex_code).unwrap()) // Here we map each line to the same hex code
        .collect();
    return palette;
}

fn parse_hex_str(str: &str) -> Result<[u8; 3], String> {
    let pat = Regex::new("^#[0-9a-fA-F]{6}$").unwrap();
    if !pat.is_match(str) {
        return Err(format!("{} is not a valid hex-color", str));
    }
    let r = u8::from_str_radix(&str[1..3], 16)
        .map_err(|_| "Invalid red value")
        .unwrap();
    let g = u8::from_str_radix(&str[3..5], 16)
        .map_err(|_| "Invalid green value")
        .unwrap();
    let b = u8::from_str_radix(&str[5..7], 16)
        .map_err(|_| "Invalid blue value")
        .unwrap();

    println!("{r}, {g}, {b}");
    Ok([r, g, b])   // Convert the hex string to a RGB color
}