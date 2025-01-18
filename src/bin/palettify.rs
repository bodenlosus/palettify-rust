use std::path::PathBuf;

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
        palettify::single_video_file(&cli.input_path, &cli.output_path, &cli.palette_path, cli.exponent);
        return;
    }

    if cli.dir {
        palettify::multi_file(&cli.palette_path, &cli.input_path, &cli.output_path, cli.exponent);
    } else {
        palettify::single_file(&cli.palette_path, &cli.input_path, &cli.output_path, cli.exponent);
    }
}