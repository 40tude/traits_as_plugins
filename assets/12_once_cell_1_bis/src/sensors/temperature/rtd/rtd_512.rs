// rtd_512.rs
use crate::sensors::temperature::temperature_sensor::TemperatureSensor;
use crate::sensors::TEMPERATURE_SENSOR_REGISTRY;

pub struct Rtd512; // camel case => no _

impl TemperatureSensor for Rtd512 {
    fn get_temp(&self) -> f64 {
        let temp: f64 = rand::random_range(256.0..512.0);
        temp
    }
}

/// Explicit registration function
pub fn register() {
    TEMPERATURE_SENSOR_REGISTRY.register_sensor("Rtd_type_512", || Box::new(Rtd512));
}
