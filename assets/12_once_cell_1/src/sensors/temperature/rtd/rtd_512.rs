use crate::sensors::temperature::temperature_sensor::TemperatureSensor;
use crate::sensors::temperature::temperature_sensor;

pub struct Rtd; // camel case => no _

impl TemperatureSensor for Rtd {
    fn get_temp(&self) -> f64 {
        let temp: f64 = rand::random_range(256.0..512.0);
        temp
    }
}

/// Explicit registration function, called from lib.rs
pub fn register() {
    temperature_sensor::register_sensor("Rtd_type_512", || Box::new(Rtd));
}
