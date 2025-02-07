mod cli;
mod image_processing;
mod palette;
mod worker;
mod video;
mod resolution;

pub use cli::Cli;
pub use worker::{single_file, multi_file, single_video_file};