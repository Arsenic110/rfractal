use std::{env, fs, path::Path, process::Command};
use rayon::prelude::*;

#[allow(unreachable_code)]
fn main() {

    //create default settings
    let mut config = Config { 
        x: -0.743643887037158704752191506114774,
        y: 0.131825904205311970493132056385139,
        img_width: 1280,
        img_height: 1280,
        frames: 320, 
        start_zoom: 400.0, 
        zoom_step: 1.1,
        log_base: 2.0,
        start_frame: 0,
    };

    /*
    seahorse valley
    x: -0.743643887037158704752191506114774,
    y: 0.131825904205311970493132056385139,

    tante renate
    x: -0.7746806106269039,
    y: -0.1374168856037867,
    
     */

    //collect commandline args into a vec
    let args: Vec<String> = env::args().collect();

    //set all the config settings according to commandline args
    for (i, arg) in args.iter().enumerate() {
        match arg.as_str() {
            "--x" => config.x = args[i + 1].parse().unwrap(),
            "--y" => config.y = args[i + 1].parse().unwrap(),
            "-x" => config.x = args[i + 1].parse().unwrap(),
            "-y" => config.y = args[i + 1].parse().unwrap(),
            "--img_width" => config.img_width = args[i + 1].parse().unwrap(),
            "--img_height" => config.img_height = args[i + 1].parse().unwrap(),
            "--frames" => config.frames = args[i + 1].parse().unwrap(),
            "--start_zoom" => config.start_zoom = args[i + 1].parse().unwrap(),
            "--zoom_step" => config.zoom_step = args[i + 1].parse().unwrap(),
            "--log_base" => config.log_base = args[i + 1].parse().unwrap(),
            "--start_frame" => config.start_frame = args[i + 1].parse().unwrap(),
            _ => {}
        }
    }

    let frame_path = "frames";
    let video_path = "output";

    //create the directory if it does not exist
    if !Path::new(frame_path).exists() {
        match fs::create_dir_all(frame_path) {
            Ok(_) => println!("created output directory for you."),
            Err(e) => eprintln!("dailed to create output directory for you, do it manually: {}", e),
        }
    } 
    if !Path::new(video_path).exists() {
        match fs::create_dir_all(video_path) {
            Ok(_) => println!("created output directory for you."),
            Err(e) => eprintln!("dailed to create output directory for you, do it manually: {}", e),
        }
    } 

    //use rayon to create parallel iterators
    (config.start_frame..(config.frames + config.start_frame)).into_par_iter().for_each(|frame| {
        //calculate zoom for current frame
        let zoom = config.start_zoom * config.zoom_step.powf(frame as f64);
        //this is a rough approx. for iterations at certain depths - there's no winning here.
        //there is always either too little, or too much samples.
        let iterations = (zoom.log(config.log_base).powf(2.0)) as u32;

        println!("Now rendering frame {frame}@{0}+i{iterations}", zoom as u32);

        //initialize an image buffer
        let mut imgbuf: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = 
            image::ImageBuffer::new(config.img_width, config.img_height);

        //iterate over each pixel and assign it a brightness value
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            //calculate the complex number corresponding to each pixel in the frame
            let c_real = config.x + (x as f64 - config.img_width as f64 / 2.0) / zoom;
            let c_img = config.y + (y as f64 - config.img_height as f64 / 2.0) / zoom;

            //mandelbrot fractals start wit z = 0+0i. for julia sets, change this.
            let mut z_real = 0.0;
            let mut z_img = 0.0;

            //escape time calculation method. 
            //it's the standard - but slow.
            let mut escape_iteration = 0;
            for i in 0..iterations {
                //formula for multiplying complex numbers
                let real = z_real * z_real - z_img * z_img + c_real;
                let img = z_real * z_img * 2.0 + c_img; 

                //if we reach escape, break
                if real * real + img * img > 4.0 {
                    escape_iteration = i;
                    break;
                }

                //otherwise, assign values for the next iteration
                z_real = real;
                z_img = img;
            }

            let (r, g, b) = get_pixel_color(z_real, z_img, escape_iteration, iterations);

            *pixel = image::Rgb([r, g, b]); //apply the color to the pixel
        }
        //save the image in the correct location! and stinky unwrap because we kinda just wanna crash if it goes wrong
        imgbuf.save(format!("{frame_path}/frame{frame}.png")).unwrap();
    });

    if config.frames < 2 {
        return;
    }

    //we want to glue frames together into a video.
    //intuitively, i just reach for ffmpeg because im a commandline freak.
    let ffmpeg_status = Command::new("ffmpeg")     //it just means you gotta have ffmpeg in your PATH
    .args([
        "-y", //always overwrite
        "-framerate", "30",     //todo: add options for this
        "-i", &format!("{frame_path}/frame%d.png"), 
        "-c:v", "libx264",      //video codec
        "-pix_fmt", "yuv420p",  //pixel format
        "-s", &format!("{0}x{1}", config.img_width / 2, config.img_height / 2), //video dimensions + antialiasing
        &format!("{video_path}/out.mp4")
    ])
    .status()
    .expect("failed to execute ffmpeg");

    if ffmpeg_status.success() {
        println!("saved as: {video_path}/out.mp4");
    } else {
        eprintln!("ffmpeg failed with status: {:?}", ffmpeg_status);
    }

}

//chatgpt ass lol
//im not doing this myself
fn get_pixel_color(z_real: f64, z_img: f64, escape_iteration: u32, iterations: u32) -> (u8, u8, u8) {
    if escape_iteration >= iterations || escape_iteration <= 0 {
        return (0, 0, 0); // Black for points that do not escape
    }

    let zn = z_real * z_real + z_img * z_img;//.sqrt(); // magnitude (distance from origin)
    let smooth_iter = escape_iteration as f64 + 1.0 - zn.log2().log2(); // smooth the iteration
    let norm = smooth_iter / iterations as f64; // normalize to [0.0, 1.0]
    let h = 360.0 * norm; // map to hue from 0 to 360

    let s = 0.7; // saturation
    let l: f64 = 0.5; // lightness
    let _a = s * l.min(1.0 - l); // adjust luminance range

    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let r_prime = if h < 60.0 { c } else if h < 120.0 { x } else if h < 180.0 { 0.0 } else if h < 240.0 { 0.0
        } else if h < 300.0 { x } else { c };
    let g_prime = if h < 60.0 { x } else if h < 120.0 { c } else if h < 180.0 { c } else if h < 240.0 { x }
        else if h < 300.0 { 0.0 } else { 0.0 };
    let b_prime = if h < 60.0 { 0.0 } else if h < 120.0 { 0.0 } else if h < 180.0 { x } else if h < 240.0 { c
        } else if h < 300.0 { c } else { x };

    let r = ((r_prime + m) * 255.0).round() as u8;
    let g = ((g_prime + m) * 255.0).round() as u8;
    let b = ((b_prime + m) * 255.0).round() as u8;

    (r, g, b)
}

#[derive(Debug, Clone)]
struct Config {
    x: f64,
    y: f64,
    img_width: u32,
    img_height: u32,
    frames: u32,
    start_zoom: f64,
    zoom_step: f64,
    log_base: f64,
    start_frame: u32,
}
