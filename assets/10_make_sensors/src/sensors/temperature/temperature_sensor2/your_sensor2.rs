// your_sensor2.rs
use crate::sensors::temperature::temperature_sensor::TempSensor;
pub struct TempSensor02;

impl TempSensor for TempSensor02 {
    // ! It is &self NOT self
    fn get_temp(&self) -> f64 {
        242.0
    }
}
