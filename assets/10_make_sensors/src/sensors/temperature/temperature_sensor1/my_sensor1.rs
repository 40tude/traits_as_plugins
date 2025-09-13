// my_sensor1.rs
use crate::sensors::temperature::temperature_sensor::TempSensor;
pub struct TempSensor01;

impl TempSensor for TempSensor01 {
    fn get_temp(&self) -> f64 {
        142.0
    }
}
