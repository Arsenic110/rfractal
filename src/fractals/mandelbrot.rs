use image::Rgb;
use crate::fractals::Fractal;

pub struct Mandelbrot {
    pub max_iter: u32,
}

impl Fractal for Mandelbrot {
    fn color_at(&self, x: f64, y: f64) -> Rgb<u8> {
        //mandelbrot fractals start wit z = 0+0i. for julia sets, change this.
        let mut z_real = 0.0;
        let mut z_img = 0.0;

        //escape time calculation method. 
        //it's the standard - but slow.
        let mut escape_iteration = 0;
        for i in 0..self.max_iter {
            //formula for multiplying complex numbers
            let real = z_real * z_real - z_img * z_img + x;
            let img = z_real * z_img * 2.0 + y; 

            //if we reach escape, break
            if real * real + img * img > 4.0 {
                escape_iteration = i;
                break;
            }

            //otherwise, assign values for the next iteration
            z_real = real;
            z_img = img;
        }

        // //simple colorization function.
        Mandelbrot::map_color(escape_iteration, self.max_iter)
    }
}