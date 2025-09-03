// use crate::input_sensors::{register_sensor, temp_sensor::TempSensor};
use crate::input_sensors::temp_sensors::TempSensor;
use crate::input_sensors::temp_sensors::register_sensor;

pub struct TempSensor02;

impl TempSensor for TempSensor02 {
    fn get_temp(&self) -> f64 {
        242.0
    }
}

/// Explicit registration function, called from lib.rs
pub fn register() {
    register_sensor("temp2", || Box::new(TempSensor02));
}
