use crate::input_sensors::temp_sensors::TempSensor;
pub struct TempSensor02;

impl TempSensor for TempSensor02 {
    // ! Compare with 014_traits_for_plugins. Now it is &self NOT self
    fn get_temp(&self) -> f64 {
        242.0
    }
}
