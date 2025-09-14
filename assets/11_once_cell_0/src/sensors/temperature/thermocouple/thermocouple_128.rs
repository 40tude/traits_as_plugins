use crate::sensors::temperature::temperature_sensor::{self, TemperatureSensor};

pub struct Thermocouple; // camel case => no _

impl TemperatureSensor for Thermocouple {
    fn get_temp(&self) -> f64 {
        let temp: f64 = rand::random_range(0.0..128.0);
        temp
    }
}
// Called from thermocouple::register()
// Explicit registration function with the followings arguments
//
// A name                 : "Thermocouple_type_128" — the key we will use later to look for the sensor
// A constructor function : || Box::new(TempSensor01) — a closure that builds a new instance of this sensor
// temperature_sensor::register_sensor() stores this name and constructor in the global registry TEMPERATURE_SENSOR_REGISTRY
// Read temperature_sensor.rs
//      pub static SENSOR_REGISTRY: Lazy<Mutex<HashMap<&'static str, Constructor>>> = ...
//      The registry is a HashMap inside a Mutex (thread-safe).
pub fn register() {
    temperature_sensor::register_sensor("Thermocouple_type_128", || Box::new(Thermocouple));
}
