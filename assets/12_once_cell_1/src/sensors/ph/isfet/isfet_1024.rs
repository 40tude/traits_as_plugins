// use crate::input_sensors::{register_sensor, temp_sensor::TempSensor};
use crate::sensors::ph::ph_sensor;
use crate::sensors::ph::ph_sensor::PhSensor;

pub struct IsFET1024; // camel case => no _

impl PhSensor for IsFET1024 {
    fn get_ph(&self) -> f64 {
        let ph: f64 = rand::random_range(6.0..7.0);
        ph
    }
}

/// Explicit registration function, called from lib.rs
pub fn register() {
    ph_sensor::register_sensor("IsFET_type_1024", || Box::new(IsFET1024));
}
