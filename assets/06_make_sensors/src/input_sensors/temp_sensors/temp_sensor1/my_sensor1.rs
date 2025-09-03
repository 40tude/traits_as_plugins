use crate::input_sensors::temp_sensors::TempSensor;
pub struct TempSensor01;

impl TempSensor for TempSensor01 {
    // ! Compare with 014_traits_for_plugins. Now it is &self NOT self
    fn get_temp(&self) -> f64 {
        142.0
    }
}
