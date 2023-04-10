use crate::signals::Wave;

pub struct Sine {
    period: f32,
    amplitude: f32,
    offset: f32,
}

impl Sine {
    pub fn new(period: f32, amplitude: f32, offset: f32) -> Self {
        Self {
            period,
            amplitude,
            offset,
        }
    }
}

impl Wave for Sine {
    fn compute(&self, t: f32) -> f32 {
        let pi = std::f32::consts::PI;

        (self.amplitude * f32::sin(2.0 * pi * (1.0 / self.period) * t)) + self.offset
    }
}
