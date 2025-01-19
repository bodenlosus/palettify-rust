use std::time::Instant;

use clap::Parser;
use palettify::Cli;

fn main() {
    let cli = Cli::parse();
    
    if !cli.input_path.exists() {
        eprintln!("Error: Input file {:?} does not exist.", cli.input_path);
        return;
    }
    if !cli.palette_path.exists() {
        eprintln!("Error: Palette file {:?} does not exist.", cli.palette_path);
        return;
    }
    if !cli.palette_path.is_file() {
        eprintln!("Error: Palette file {:?} is not a file.", cli.palette_path);
        return;
    }
    
    if cli.video {
        let elapsed = timed!(
            palettify::single_video_file(&cli.input_path, &cli.output_path, &cli.palette_path, cli.exponent)
        );
        println!("Task took {} seconds", elapsed.as_secs());
        return;
    }

    if cli.dir {
        let elapsed = timed!(
            palettify::multi_file(&cli.palette_path, &cli.input_path, &cli.output_path, cli.exponent)
        );
        println!("Task took {} seconds", elapsed.as_secs())
        
        
    } else {
        let elapsed = timed!(
        palettify::single_file(&cli.palette_path, &cli.input_path, &cli.output_path, cli.exponent)
        );
        println!("Task took {} seconds", elapsed.as_secs())
    }
}

#[macro_export]
macro_rules! timed {
    ( $( $x:expr ),* ) => {
        {
            let now = Instant::now();
            $(
                $x;
            )*
            let elapsed = now.elapsed();
            elapsed
        }
    };
}