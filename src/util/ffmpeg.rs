use std::process::Command;

pub fn create_video(width: u32, height: u32, frame_path: &str, video_path: &str) {
    //we want to glue frames together into a video.
    let ffmpeg_status = Command::new("ffmpeg")     //it just means you gotta have ffmpeg in your PATH
    .args([
        "-y", //always overwrite
        "-framerate", "30",     //todo: add options for this
        "-i", &format!("{frame_path}/frame%d.png"), 
        "-c:v", "libx264",      //video codec
        "-pix_fmt", "yuv420p",  //pixel format
        "-s", &format!("{0}x{1}", width / 2, height / 2), //video dimensions + antialiasing
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