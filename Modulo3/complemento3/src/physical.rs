pub struct System {}

impl Default for System {
    fn default() -> Self {
        Self {}
    }
}

impl System {
    pub fn sensor(&self, t: f32) -> f32 {
        const TAU: f32 = 0.1;

        1.0 - f32::exp(-t / TAU)
    }
}

pub mod controller {
    pub const P: f32 = 5.0;
}
