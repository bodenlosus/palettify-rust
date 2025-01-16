extern crate image;
extern crate regex;
extern crate clap;
extern crate termion;

use clap::Parser;
use regex::Regex;
use image::{ImageReader, Rgb};
use std::{fs, path::{Path, PathBuf}};
use termion::color;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The input image file path
    #[arg(short, long, value_name = "INPUT", help = "Path to the input image")]
    input_path: PathBuf,

    /// The output image file path
    #[arg(short, long, value_name = "OUTPUT", default_value = "o.png", help = "Path to the output image (default: o.png)")]
    output_path: PathBuf,

    /// The palette file path
    #[arg(short, long, value_name = "PALETTE", help = "Path to the palette file")]
    palette_path: PathBuf,

    /// The exponent value for processing (default: 2)
    #[arg(short, long, value_name = "EXPONENT", default_value_t = 15, help = "Exponent for processing. Bigger Exponent > more quantization (default: 15)")]
    exponent: i32,

    #[arg(long, short, action)]
    dir: bool,
}

fn main() {
    let cli = Cli::parse();

    // Check if the input file exists
    if !cli.input_path.exists() {
        eprintln!("Error: Input file {:?} does not exist.", cli.input_path);
        return;
    }

    // Check if the palette file exists
    if !cli.palette_path.exists() {
        eprintln!("Error: Palette file {:?} does not exist.", cli.palette_path);
        return;
    }

    if !cli.palette_path.is_file() {
        eprintln!("Error: Palette file {:?} is not a file.", cli.palette_path);
        return;
    }


    if cli.dir {
        multi_file(&cli.palette_path, &cli.input_path, &cli.output_path, cli.exponent);
        return;
    }
    single_file(&cli.palette_path, &cli.input_path, &cli.output_path, cli.exponent);

}

fn single_file(palette_path: &Path, input_path: &Path, output_path:&Path, exponent: i32) {
    if !input_path.is_file() {
        eprintln!("Error: Input file {:?} is not a file.", input_path);
        return;
    }

    if let Some(parent_dir) = output_path.parent() {
        if !parent_dir.exists() {
            if let Err(e) = fs::create_dir_all(parent_dir) {
                eprintln!("Error creating directories for output path: {}", e);
                return;
            }
        }
    }
    let palette = read_palette(palette_path);

    process_image(&palette, &input_path, &output_path, exponent);
}

fn multi_file(palette_path: &Path, input_path: &Path, output_path:&Path, exponent: i32) {
    if !input_path.is_dir() {
        eprintln!("Error: Input file {:?} is not a directory.", input_path);
        return;
    }

    if !output_path.exists() {
        if let Err(e) = fs::create_dir_all(output_path) {
            eprintln!("Error creating directories for output path: {}", e);
            return;
        }
    }

    if !output_path.is_dir(){
        eprintln!("Error: Output path {:?} is not a directory.", output_path);
        return;
    }


    let palette = read_palette(palette_path);

    for entry in fs::read_dir(input_path).unwrap() {
        match entry {
            Ok(entry) => {
                if entry.path().is_file() {
                    let input_file = entry.path();
                    let mut output_file = output_path.join(entry.file_name());
    
                    // Prepend "out" to the output filename
                    if let Some(file_name) = output_file.file_name() {
                        let new_file_name = format!("palettify-{}", file_name.to_string_lossy());
                        output_file.set_file_name(new_file_name);
                    }
                    println!("Processing {}...", input_file.display());
                    process_image(&palette, &input_file, &output_file, exponent);
                }
            }
            Err(e) => eprintln!("Error reading directory entry: {}", e),
        }
    }
}

fn process_image(palette: &Vec<[u8; 3]>, input_path:&Path, output_path:&Path, exponent: i32) {
    let dyn_img = match ImageReader::open(input_path) {
        Ok(r) => r,
        Err(e) => panic!("Error opening image: {}", e),
    };
    let mut img = dyn_img.decode().unwrap().to_rgb8();

    // Set each pixel to a color
    for (_, _, pixel) in img.enumerate_pixels_mut() {          // Blue channel
        let r = (pixel[0]) as u8;
        let g = (pixel[1]) as u8;
        let b = (pixel[2]) as u8;
        *pixel = Rgb(interpolate([r, g, b], &palette, exponent));
    }

    // Save the image to a file
    match img.save(output_path) {
        Ok(_) => println!("Image saved as {}", output_path.display()),
        Err(e) => println!("Error saving image: {}", e),
    }
}

fn interpolate(color: [u8; 3], palette: &[[u8; 3]], exponent: i32) -> [u8; 3] {
    use std::f32::INFINITY;

    match palette.len() {
        0 => return [0; 3],
        1 => return palette[0],
        _ => {}
    }

    // Pre-allocate arrays for better performance
    let mut distances = Vec::with_capacity(palette.len());
    let mut min_r: f32 = INFINITY;
    let mut max_r: f32 = 0.0;
    
    // Calculate all distances in a single pass
    for &pcolor in palette {
        let dist: f32 = (0..3)
            .fold(0.0, |acc, i| {
                acc + (color[i] as f32 - pcolor[i] as f32).powi(2)
            });
        min_r = min_r.min(dist);
        max_r = max_r.max(dist);
        distances.push(dist);
    }

    // Early return for identical distances
    if (max_r - min_r).abs() < f32::EPSILON {
        return palette[0];
    }

    let range_inv = 1.0 / (max_r - min_r);
    let mut weighted_sum = [0.0f32; 3];
    let mut sum = 0.0f32;

    // Combine the zip and fold operations into a single loop
    for (dist, &pcolor) in distances.iter().zip(palette) {
        let weight = ((max_r - dist) * range_inv).powi(exponent);
        sum += weight;
        
        // Unrolled loop for better performance
        weighted_sum[0] += weight * pcolor[0] as f32;
        weighted_sum[1] += weight * pcolor[1] as f32;
        weighted_sum[2] += weight * pcolor[2] as f32;
    }

    let sum_inv = 1.0 / sum;
    [
        (weighted_sum[0] * sum_inv).clamp(0.0, 255.0).round() as u8,
        (weighted_sum[1] * sum_inv).clamp(0.0, 255.0).round() as u8,
        (weighted_sum[2] * sum_inv).clamp(0.0, 255.0).round() as u8,
    ]
}

fn read_palette(path: &Path) -> Vec<[u8; 3]>{
    println!("Reading palette {}", path.display());
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
    Ok(color)   // Convert the hex string to a RGB color
}