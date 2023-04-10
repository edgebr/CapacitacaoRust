pub trait Wave {
    fn compute(&self, t: f32) -> f32;
}

pub mod sawtooth;
pub mod sine;
pub mod square;

mod other {
    use super::*;
}

pub use sawtooth::SawTooth;
