use crate::{Vector3,WindModel};
use crate::types::Real;

pub struct PowerWind<T: Real> {
    u_r: T,
    z_r: T,
    alpha: T,
    bearing: T,
}

impl<T: Real> PowerWind<T> {
    const ALPHA_TYPICAL: T = T::from(0.143);
    
    pub fn new_with_alpha(u_r: T, z_r: T, bearing: T, alpha: T) -> Self {
        PowerWind { u_r, z_r, bearing, alpha }
    }
    
    pub fn new(u_r: T, z_r: T, bearing: T) -> Self {
        PowerWind::new_with_alpha(u_r, z_r, bearing, PowerWind::ALPHA_TYPICAL)
    }
}

impl<T: Real> WindModel<T> for PowerWind<T> {
    fn get_wind(&self, position: &Vector3<T>) -> Vector3<T> {
        let velocity = self.u_r * (position.z / self.z_r).powf(self.alpha);
        let bearing_rad = self.bearing.to_radians();
        Vector3::new(
            velocity * bearing_rad.cos(),
            velocity * bearing_rad.sin(),
            T::zero())
    }
    
    fn step(&mut self, _delta_t: T) {}
}

#[test]
fn test_powercalc() {
    use approx::assert_relative_eq;
    
    const U_R: f64 = 10.0;
    const Z_R: f64 = 10.0;
    const ALPHA: f64 = 0.143;
    
    let wind_model = PowerWind::new(U_R,Z_R,0.0);
    
    for height_idx in 0..20 {
        let height = height_idx as f64 * 0.1;
        
        let expected_result = U_R * (height/Z_R).powf(ALPHA);
    
        let wind = wind_model.get_wind(&Vector3::new(0.0,0.0,height));
        assert_relative_eq!(wind.x,expected_result);
    }
}
