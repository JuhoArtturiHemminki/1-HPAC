use crate::fractal_feedback::{PHI, H_C};

pub struct GoldilocksLimiter {
    pub target_temp_raw: f64,
    pub current_output_mw: f64,
}

impl GoldilocksLimiter {
    pub const fn new(target: f64) -> Self {
        Self {
            target_temp_raw: target,
            current_output_mw: 0.0,
        }
    }

    pub fn balance(&mut self, current_temp: f64, drift: &mut f64) -> f64 {
        let delta = current_temp - self.target_temp_raw;
        if delta > 0.0 {
            *drift += (delta * PHI) / H_C;
            self.current_output_mw = delta * 500.0;
        } else {
            *drift -= (delta.abs() * PHI) / H_C;
            self.current_output_mw = 0.0;
        }
        self.current_output_mw
    }
}

