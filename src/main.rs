#![allow(dead_code)]

use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::fractals::*;
use crate::util::*;

mod fractals;
mod util;

#[allow(unreachable_code)]
fn main() {
    /*
        my personal collection of cool places:

        seahorse valley
        x: -0.743643887037158704752191506114774,
        y: 0.131825904205311970493132056385139,

        tante renate
        x: -0.7746806106269039,
        y: -0.1374168856037867,
     */

    //create default settings
    let config = Config::new();
    util::file::validate_fs(&config);

    (config.start_frame..(config.frames + config.start_frame)).for_each(|frame| {
        //calculate zoom for current frame
        let zoom = config.start_zoom * config.zoom_step.powf(frame as f64);
        //approximate iterations for escape time at certain depths
        let iterations = (zoom.log(config.log_base).powf(2.0)) as u32;

        println!("Now rendering frame {frame}@{0}+i{iterations}", zoom as u32);

        //initialize an image buffer
        let mut imgbuf: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = 
            image::ImageBuffer::new(config.img_width, config.img_height);

        let frac = 
            DoublePendulum::default();

        let (width, height) = imgbuf.dimensions();
        let width = width as usize;
        let height = height as usize;

        let counter = AtomicUsize::new(0);
        let total_pixels = width * height;
        
        imgbuf.as_flat_samples_mut().samples
            .par_chunks_mut(width * 3)
            .enumerate()
            .for_each(|(y, row)| {
                for x in 0..width {
                    let offset = x * 3 ;
                    let pixel = &mut row[offset..offset+3];
                    // apply filter using input_pixel and neighbors

                    let c_real = config.x + (x as f64 - config.img_width as f64 / 2.0) / zoom;
                    let c_img = config.y + (y as f64 - config.img_height as f64 / 2.0) / zoom;

                    pixel.copy_from_slice(&frac.color_at(c_real, c_img).0);

                    // increment counter
                    let done = counter.fetch_add(1, Ordering::Relaxed) + 1;
                    if done % 100_000 == 0 {
                        println!("Progress: {:.2}%", done as f64 / total_pixels as f64 * 100.0);
                    }
                }
            });

        //save the image in the correct location! and stinky unwrap because we kinda just wanna crash if it goes wrong
        imgbuf.save(format!("{}/frame{frame}.png", config.frame_path)).unwrap();
    });

    //if there's more than 2 frames, we can render a video using ffmpeg.
    if config.frames >= 2 {
        ffmpeg::create_video(
            config.img_width, 
            config.img_height, 
            config.frame_path.as_str(), 
            config.video_path.as_str()
        );
    }
}




