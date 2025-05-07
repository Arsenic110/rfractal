use std::env;
use std::process::Command;
use rayon::prelude::*;

fn main() {

    //create default settings
    let mut config = Config { 
        x: -0.743643887037158704752191506114774, 
        y: 0.131825904205311970493132056385139, 
        img_width: 3840, 
        img_height: 2160,
        iterations: 255, 
        frames: 50, 
        start_zoom: 150.0, 
        zoom_step: 1.2,
    };

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
            "--iterations" => config.iterations = args[i + 1].parse().unwrap(),
            "--frames" => config.frames = args[i + 1].parse().unwrap(),
            "--start_zoom" => config.start_zoom = args[i + 1].parse().unwrap(),
            "--zoom_step" => config.zoom_step = args[i + 1].parse().unwrap(),
            _ => {}
        }
    }

    (0..config.frames).into_par_iter().for_each(|frame| {
        let zoom = config.start_zoom * config.zoom_step.powf(frame as f64);
        let iterations = (zoom.log(1.5) * 50.0) as u32;

        println!("Now rendering frame {frame}@{0}+i{iterations}", zoom as u32);

        //initialize an image
        let mut imgbuf: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = 
            image::ImageBuffer::new(config.img_width, config.img_height);

        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let c_real = config.x + (x as f64 - config.img_width as f64 / 2.0) / zoom;
            let c_img = config.y + (y as f64 - config.img_height as f64 / 2.0) / zoom;

            let mut z_real = 0.0;
            let mut z_img = 0.0;

            let mut escape_iteration = 0;
            for i in 0..iterations {
                let real = z_real * z_real - z_img * z_img + c_real;
                let img = z_real * z_img * 2.0 + c_img; 

                if real * real + img * img > 4.0 {
                    escape_iteration = i;
                    break;
                }
                z_real = real;
                z_img = img;
            }

            let r = (escape_iteration as f64 / iterations as f64 * 255.0) as u8;
            *pixel = image::Rgb([r, r, r]);
        }
        imgbuf.save(format!("output/frame{frame}.png")).unwrap();
    });

    let ffmpeg_status = Command::new("ffmpeg")
    .args([
        "-y",
        "-framerate", "10",
        "-i", "output/frame%d.png",
        "-c:v", "libx264",
        "-pix_fmt", "yuv420p",
        "-s", "1920x1080",
        "output/out.mp4"
    ])
    .status()
    .expect("failed to execute ffmpeg");

    if ffmpeg_status.success() {
        println!("saved as: output/out.mp4");
    } else {
        eprintln!("ffmpeg failed with status: {:?}", ffmpeg_status);
    }

}

#[derive(Debug, Clone)]
struct Config {
    x: f64,
    y: f64,
    img_width: u32,
    img_height: u32,
    iterations: u32,
    frames: u32,
    start_zoom: f64,
    zoom_step: f64
}
