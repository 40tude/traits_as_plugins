// isfet_1024.rs
use crate::sensors::ph::ph_sensor::PhSensor;
use crate::sensors::PH_SENSOR_REGISTRY;

pub struct IsFET1024; // camel case => no _

impl PhSensor for IsFET1024 {
    fn get_ph(&self) -> f64 {
        let ph: f64 = rand::random_range(6.0..7.0);
        ph
    }
}

// Explicit registration function
pub fn register() {
    PH_SENSOR_REGISTRY.register_sensor("IsFET_type_1024", || Box::new(IsFET1024));
}
