use crate::signals::Wave;

pub struct SawTooth {
    period: f32,
    amplitude: f32,
    offset: f32,
}

impl SawTooth {
    pub fn new(period: f32, amplitude: f32, offset: f32) -> Self {
        Self {
            period,
            amplitude,
            offset,
        }
    }
}

impl Wave for SawTooth {
    fn compute(&self, t: f32) -> f32 {
        let now = t % self.period;

        (self.amplitude * (now / self.period)) + self.offset
    }
}
