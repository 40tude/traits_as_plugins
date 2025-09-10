// your_sensor2.rs
use crate::sensors::temp::temp_sensor::TempSensor;
pub struct TempSensor02;

impl TempSensor for TempSensor02 {
    fn get_temp(self) -> f64 {
        242.0
    }
}
