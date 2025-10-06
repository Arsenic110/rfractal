use image::Rgb;
use crate::fractals::Fractal;
use std::f64::consts::PI;

#[derive(Clone)]
struct State {
    th1: f64,
    th2: f64,
    w1: f64,
    w2: f64,
}

impl State {
    fn add_scaled(&self, other: &State, s: f64) -> State {
        State {
            th1: self.th1 + other.th1 * s,
            th2: self.th2 + other.th2 * s,
            w1:  self.w1  + other.w1  * s,
            w2:  self.w2  + other.w2  * s,
        }
    }
    fn sub(&self, other: &State) -> State {
        State {
            th1: self.th1 - other.th1,
            th2: self.th2 - other.th2,
            w1:  self.w1  - other.w1,
            w2:  self.w2  - other.w2,
        }
    }
    fn norm(&self) -> f64 {
        (self.th1*self.th1 + self.th2*self.th2 + self.w1*self.w1 + self.w2*self.w2).sqrt()
    }
}

pub struct DoublePendulum {
    pub max_steps: usize,    // number of integration steps
    pub dt: f64,             // timestep
    pub renorm_every: usize, // renormalization interval (steps)
    pub delta0: f64,         // initial perturbation magnitude
    pub g: f64,
    pub lyap_scale: f64,     // scale/clamp for color mapping
}

impl Default for DoublePendulum {
    fn default() -> Self {
        DoublePendulum { 
            max_steps: 900, //500-1000
            dt: 0.005,      //.004-.006
            renorm_every: 50, //50
            delta0: 1e-8, //can't be zero
            g: 9.81, //9.81
            lyap_scale: 1.5 //2-5
        }
    }
}

impl DoublePendulum {
    fn deriv(&self, s: &State) -> State {
        let g = self.g;
        let th1 = s.th1;
        let th2 = s.th2;
        let w1 = s.w1;
        let w2 = s.w2;

        let delta = th1 - th2;
        let den = 2.0 - (2.0 * delta).cos(); // 2 - cos(2*delta)

        //th1:
        let num1 = -g*(2.0*th1.sin()) - g*(th1 - 2.0*th2).sin()
                   - 2.0*(delta).sin()*(w2*w2 + w1*w1*(delta).cos());
        let a1 = num1 / den;

        //th2:
        let num2 = 2.0*(delta).sin()*(w1*w1 + g*th1.cos() + w2*w2*(delta).cos());
        let a2 = num2 / den;

        State {
            th1: w1,
            th2: w2,
            w1: a1,
            w2: a2,
        }
    }

    //RK4 integration step
    fn rk4_step(&self, s: &State, dt: f64) -> State {
        let k1 = self.deriv(s);
        let s2 = s.add_scaled(&k1, dt * 0.5);
        let k2 = self.deriv(&s2);
        let s3 = s.add_scaled(&k2, dt * 0.5);
        let k3 = self.deriv(&s3);
        let s4 = s.add_scaled(&k3, dt);
        let k4 = self.deriv(&s4);

        State {
            th1: s.th1 + (dt/6.0) * (k1.th1 + 2.0*k2.th1 + 2.0*k3.th1 + k4.th1),
            th2: s.th2 + (dt/6.0) * (k1.th2 + 2.0*k2.th2 + 2.0*k3.th2 + k4.th2),
            w1:  s.w1  + (dt/6.0) * (k1.w1  + 2.0*k2.w1  + 2.0*k3.w1  + k4.w1),
            w2:  s.w2  + (dt/6.0) * (k1.w2  + 2.0*k2.w2  + 2.0*k3.w2  + k4.w2),
        }
    }
}

impl Fractal for DoublePendulum {
    fn color_at(&self, px: f64, py: f64) -> Rgb<u8> {
        //map px & py to angle ranges:
        let th1_min = -PI;
        let th1_max = PI;
        let th2_min = -PI;
        let th2_max = PI;

        let th1 = th1_min + (th1_max - th1_min) * px;
        let th2 = th2_min + (th2_max - th2_min) * py;
        let base = State { th1, th2, w1: 0.0, w2: 0.0 };

        //small initial perturbation along th1 direction
        let pert = State { th1: self.delta0, th2: 0.0, w1: 0.0, w2: 0.0 };
        let mut s = base.clone();
        let mut sp = base.add_scaled(&pert, 1.0);

        let mut sum_log = 0.0;
        let mut time = 0.0;
        let total_steps = self.max_steps;

        for step in 0..total_steps {
            s = self.rk4_step(&s, self.dt);
            sp = self.rk4_step(&sp, self.dt);
            time += self.dt;

            if (step % self.renorm_every) == 0 && step > 0 {
                let diff = sp.sub(&s);
                let d = diff.norm();
                if d == 0.0 { continue; }
                sum_log += (d / self.delta0).ln();
                //renormalize sp to be delta0 away from s in same direction
                let factor = self.delta0 / d;
                let unit = State {
                    th1: diff.th1 * factor,
                    th2: diff.th2 * factor,
                    w1:  diff.w1  * factor,
                    w2:  diff.w2  * factor,
                };
                sp = s.add_scaled(&unit, 1.0);

                //optional early escape
                if d > 1e2 { //extremely large separation -> give as fast-escape
                    break;
                }
            }
        }

        let lambda = if time > 0.0 { sum_log / time } else { 0.0 }; //finite-time lyapunov

        //map lambda to color: scale and clamp
        let t = (lambda / self.lyap_scale).max(0.0).min(1.0);

        let (r, g, b) = if t <= 0.5 {
            //black -> blue
            let k = t / 0.5; //0.0 -> 1.0
            let r = 0;
            let g = 0;
            let b = (255.0 * k) as u8;
            (r, g, b)
        } else {
            // blue -> white
            let k = (t - 0.5) / 0.5; //0.0 -> 1.0
            let r = (255.0 * k) as u8;
            let g = (255.0 * k) as u8;
            let b = 255;
            (r, g, b)
        };

        Rgb([r, g, b])
    }
}
