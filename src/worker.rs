use std::{fs, path::{Path, PathBuf}, sync::{Arc, Mutex}, thread};
use crate::{image_processing, palette};

pub struct ProcessTask {
    pub input_path: PathBuf,
    pub output_path: PathBuf,
    pub exponent: i32,
}

pub fn single_file(palette_path: &Path, input_path: &Path, output_path: &Path, exponent: i32) {
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
    
    let palette = palette::read_palette(palette_path);
    image_processing::process_image(&palette, input_path, output_path, exponent);
}

pub fn multi_file(palette_path: &Path, input_path: &Path, output_path: &Path, exponent: i32) {
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
    
    if !output_path.is_dir() {
        eprintln!("Error: Output path {:?} is not a directory.", output_path);
        return;
    }
    
    let palette = palette::read_palette(palette_path);
    let mut queue: Vec<ProcessTask> = vec![];
    
    for entry in fs::read_dir(input_path).unwrap() {
        match entry {
            Ok(entry) => {
                if entry.path().is_file() {
                    let input_file = entry.path();
                    let mut output_file = output_path.join(entry.file_name());
                    
                    if let Some(file_name) = output_file.file_name() {
                        let new_file_name = format!("palettify-{}", file_name.to_string_lossy());
                        output_file.set_file_name(new_file_name);
                    }
                    
                    println!("Added {} to queue", input_file.display());
                    queue.push(ProcessTask {
                        input_path: input_file.clone(),
                        output_path: output_file.clone(),
                        exponent,
                    });
                }
            }
            Err(e) => eprintln!("Error reading directory entry: {}", e),
        }
    }
    
    batch_process(queue, palette);
}

fn worker(queue: Arc<Mutex<Vec<ProcessTask>>>, palette: &Arc<Vec<[u8; 3]>>) {
    loop {
        let task = {
            let mut queue = queue.lock().unwrap();
            queue.pop()
        };
        
        if let Some(task) = task {
            println!("Processing {}...", task.input_path.display());
            image_processing::process_image(palette, &task.input_path, &task.output_path, task.exponent);
        } else {
            break;
        }
    }
}

fn batch_process(queue: Vec<ProcessTask>, palette: Vec<[u8; 3]>) {
    let thread_count = thread::available_parallelism().unwrap().get();
    let mut active_threads = Vec::new();
    let q = Arc::new(Mutex::new(queue));
    let p = Arc::new(palette);
    
    for _ in 0..thread_count {
        let queue_clone = Arc::clone(&q);
        let palette_clone = Arc::clone(&p);
        let handle = thread::spawn(move || {
            worker(queue_clone, &palette_clone);
        });
        active_threads.push(handle);
    }
    
    for handle in active_threads {
        handle.join().unwrap();
    }
}