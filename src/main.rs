/// Matrix math
mod matrix;

/// Add curves to an edge matrix
mod curve;

/// Add 3D solids to an edge matrix
mod solid;

/// Render edges to an in-memory representation of the pixels of the screen
mod render;

/// Create image files
mod ppm;

mod parse;

/// Execute commands from a script
mod exec;

mod worker;

/// Crate-wide constants
mod consts;

use std::fs::File;

use std::io::prelude::*;
use std::sync::mpsc::channel;
use std::time::Instant;

fn main() {
    match File::open("script") {
        Err(e) => {
            panic!("Could not open file 'script'. Error: {}", e);
        },
        Ok(mut file) => {
            let mut s = String::from("");
            match file.read_to_string(&mut s) {
                Ok(_) => {
                    let (tx, rx) = channel();
                    let handle = ppm::spawn_saver(rx);
                    let start = Instant::now();
                    if let Err(msg) = exec::run_script(&s, tx) {
                        println!("Error!\n{}", msg);
                    }
                    handle.join();
                    let elapsed = start.elapsed();
                    println!("Total time: {}s {}ms", elapsed.as_secs(), elapsed.subsec_nanos() as u64 / 1000000);
//                    for handle in handles {
//                        handle.join();
//                    }
                    ppm::clean_up();
                },
                Err(e) => {
                    panic!("Error reading text in ./script: {}", e);
                }
            }
        }
    }
}
