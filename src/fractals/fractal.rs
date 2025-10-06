use image::Rgb;

pub trait Fractal {

    ///Compute fractal pixel color at (x, y) in the image coordinate space.
    fn color_at(&self, x: f64, y: f64) -> Rgb<u8>;

    fn map_color(iter: u32, max_iter: u32) -> Rgb<u8> {
        let t = iter as f64 / max_iter as f64;
        let r = (9.0 * (1.0 - t) * t * t * t * 255.0) as u8;
        let g = (15.0 * (1.0 - t) * (1.0 - t) * t * t * 255.0) as u8;
        let b = (8.5 * (1.0 - t) * (1.0 - t) * (1.0 - t) * t * 255.0) as u8;
        Rgb([r, g, b])
    }
}