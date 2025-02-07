use image::{imageops::resize, ImageReader, Rgb};
use std::{cmp::min, path::Path};

use crate::resolution::Resolutions;

pub type Image = image::ImageBuffer<Rgb<u8>, Vec<u8>>;
pub fn process_image(palette: &Vec<[u8; 3]>, input_path: &Path, output_path: &Path, exponent: i32, resolution: Resolutions) {
    let dyn_img = match ImageReader::open(input_path) {
        Ok(r) => r,
        Err(e) => panic!("Error opening image: {}", e),
    };

    let mut img: Image = dyn_img.decode().unwrap().to_rgb8();
    
    if let Some(resized_image) = img_resize(&mut img, resolution) {
        img = resized_image;
    }

    process(palette, &mut img, exponent);

    match img.save(output_path) {
        Ok(_) => println!("Image saved as {}", output_path.display()),
        Err(e) => println!("Error saving image: {}", e),
    }
}

fn img_resize(image: &Image, res: Resolutions) -> Option<Image> {
    if res == Resolutions::NONE {
        return None;
    }
    let res = res as u32;

    let (width, height) = image.dimensions();
    let min = min(width, height);

    if min < res { return None; }

    let f = res as f32 / min as f32;
    
    let nwidth = (width as f32 * f) as u32;
    let nheight = (height as f32 * f) as u32;
    return Some(resize(image, nwidth, nheight, image::imageops::FilterType::Triangle));
}
pub fn process(palette: &Vec<[u8; 3]>, img: &mut Image, exponent: i32) {
    
    img.pixels_mut().for_each(|pixel | {
        pixel.0 = interpolate(pixel.0, palette, exponent);
     });
    
}

pub fn interpolate(color: [u8; 3], palette: &[[u8; 3]], exponent: i32) -> [u8; 3] {
    use std::f32::INFINITY;
    
    match palette.len() {
        0 => return [0; 3],
        1 => return palette[0],
        _ => {}
    }
    
    let mut distances = Vec::with_capacity(palette.len());
    let mut min_r: f32 = INFINITY;
    let mut max_r: f32 = 0.0;
    
    for &pcolor in palette {
        let dist: f32 = (0..3)
            .fold(0.0, |acc, i| {
                acc + (color[i] as f32 - pcolor[i] as f32).powi(2)
            });
        min_r = min_r.min(dist);
        max_r = max_r.max(dist);
        distances.push(dist);
    }
    
    if (max_r - min_r).abs() < f32::EPSILON {
        return palette[0];
    }
    
    let range_inv = 1.0 / (max_r - min_r);
    let mut weighted_sum = [0.0f32; 3];
    let mut sum = 0.0f32;
    
    for (dist, &pcolor) in distances.iter().zip(palette) {
        let weight = ((max_r - dist) * range_inv).powi(exponent);
        sum += weight;
        
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