use crate::{Vector3,WindModel};
use crate::types::Float;

pub struct ConstantWind<T: Float> {
    wind: Vector3<T>,
}

impl<T: Float> ConstantWind<T> {
    pub fn new(wind: Vector3<T>) -> Self {
        ConstantWind {
            wind,
        }
    }
}

impl<T: Float> WindModel<T> for ConstantWind<T> {
    fn get_wind(&self, _position: &Vector3<T>) -> Vector3<T> {
        self.wind
    }
    
    fn step(&mut self, _delta_t: T) {}
}

#[test]
fn test_constant() {
    // Create random wind
    // Test output is the same
}