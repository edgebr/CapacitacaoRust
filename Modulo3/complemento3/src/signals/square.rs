use crate::signals::Wave;

pub struct SquareWave {
    period: f32,
    amplitude: f32,
    offset: f32,
}

impl SquareWave {
    pub fn new(period: f32, amplitude: f32, offset: f32) -> Self {
        Self {
            period,
            amplitude,
            offset,
        }
    }
}

impl Wave for SquareWave {
    fn compute(&self, t: f32) -> f32 {
        let now = t % self.period;
        let state = if now >= (self.period / 2.0) { 1.0 } else { 0.0 };

        (self.amplitude * state) + self.offset
    }
}
