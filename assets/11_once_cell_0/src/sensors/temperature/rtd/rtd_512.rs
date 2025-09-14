use crate::sensors::temperature::temperature_sensor::{self, TemperatureSensor};

pub struct Rtd;

impl TemperatureSensor for Rtd {
    fn get_temp(&self) -> f64 {
        let temp: f64 = rand::random_range(256.0..512.0);
        temp
    }
}

// Called from rtd::register()
pub fn register() {
    temperature_sensor::register_sensor("Rtd_type_512", || Box::new(Rtd));
}
