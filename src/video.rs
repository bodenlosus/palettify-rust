use std::{fmt::Result, path::PathBuf};

use ndarray::{ArrayBase, OwnedRepr};
use video_rs::{encode::Settings, Decoder, Encoder, Location, Time};
use rayon::prelude::*;

use crate::image_processing::interpolate;

extern crate video_rs;

type Frame = ArrayBase<OwnedRepr<u8>, ndarray::Dim<[usize; 3]>>;

pub fn process_video(input_path:&PathBuf, output_path: &PathBuf, palette:&Vec<[u8; 3]>, exponent: i32) -> Result{
    video_rs::init().unwrap();

    

    let mut decoder = Decoder::new(Location::File(input_path.clone())).expect("failed to create decoder");
    let (width, height) = decoder.size();
    let duration = Time::from_secs(1. / decoder.frame_rate());
    let settings = Settings::preset_h264_yuv420p(width as usize, height as usize, false);
    let mut encoder =
        Encoder::new(Location::File(output_path.clone()), settings).expect("failed to create encoder");

    println!("{}", decoder.frame_rate());

    let mut position = Time::zero();
    for frame in decoder.decode_iter().enumerate() {
        if let (i, Ok((_, mut frame))) = frame {
            println!("{}", i);
            manipulate_frame(&mut frame, palette, exponent);
            match encoder.encode(&frame, position){
                Ok(_) => (),
                Err(e) => {eprintln!("Error encoding frame {}: {}", i, e);continue;},  
            }
            position = position.aligned_with(duration).add();
        } else {
            break;
        }
    }

    match encoder.finish(){
        Ok(_) => (),
        Err(e) => eprintln!("Error encoding frame {}: {}", duration.as_secs(), e),
    };
    Ok(())
}



fn manipulate_frame(frame: &mut Frame, palette: &[[u8; 3]], exponent: i32) {
    let shape = frame.shape();
    let width = shape[1];
    
    frame.as_slice_mut().unwrap()
        .par_chunks_mut(width * 3)
        .for_each(|row| {
            for pixel in row.chunks_mut(3) {
                let current_color = [pixel[0], pixel[1], pixel[2]];
                let [r, g, b] = interpolate(current_color, palette, exponent);
                pixel[0] = r;
                pixel[1] = g;
                pixel[2] = b;
            }
        });
}