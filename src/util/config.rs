use std::env::args;

#[derive(Debug, Clone)]
pub struct Config {
    pub x: f64,
    pub y: f64,
    pub img_width: u32,
    pub img_height: u32,
    pub frames: u32,
    pub start_zoom: f64,
    pub zoom_step: f64,
    pub log_base: f64,
    pub start_frame: u32,

    pub frame_path: String,
    pub video_path: String,
}

impl Default for Config {
    fn default() -> Self {
        Config { 
            x: 0.5,
            y: 0.5,
            img_width: 1024,
            img_height: 1024,
            frames: 1, 
            start_zoom: 900.0, 
            zoom_step: 2.0,
            log_base: 2.0,
            start_frame: 0,
            frame_path: "frames".to_string(),
            video_path: "video_path".to_string(),
        }
    }
}

impl Config {
    pub fn new() -> Config {
        let mut config = Config::default();

        //collect commandline args into a vec
        let args: Vec<String> = args().collect();

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

        config
    }

}