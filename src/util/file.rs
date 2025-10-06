use std::{fs, path::Path};

use crate::util::Config;

pub fn validate_fs(config: &Config) -> bool {
    let frame_path = &config.frame_path;
    let video_path = &config.video_path;

    //create the directory if it does not exist
    if !Path::new(frame_path).exists() {
        match fs::create_dir_all(frame_path) {
            Ok(_) => println!("created output directory for you."),
            Err(e) => {
                eprintln!("failed to create output directory for you, do it manually: {}", e);
                return false;
            }
        }
    } 
    
    if !Path::new(video_path).exists() {
        match fs::create_dir_all(video_path) {
            Ok(_) => println!("created output directory for you."),
            Err(e) => {
                eprintln!("failed to create output directory for you, do it manually: {}", e);
                return false;
            }
        }
    } 

    true
}