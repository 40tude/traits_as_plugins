// temp_sensor.rs
pub trait TempSensor {
    fn get_temp(self) -> f64;
}
