// thermocouple_128.rs
use crate::sensors::temperature::temperature_sensor::TemperatureSensor;
use crate::sensors::TEMPERATURE_SENSOR_REGISTRY;

pub struct Thermocouple128; // camel case => no _

impl TemperatureSensor for Thermocouple128 {
    fn get_temp(&self) -> f64 {
        let temp: f64 = rand::random_range(0.0..128.0);
        temp
    }
}

// Explicit registration function
pub fn register() {
    TEMPERATURE_SENSOR_REGISTRY.register_sensor("Thermocouple_type_128", || Box::new(Thermocouple128));
}
