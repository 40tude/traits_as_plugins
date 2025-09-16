use crate::sensors::temperature::temperature_sensor;
use crate::sensors::temperature::temperature_sensor::TemperatureSensor;

pub struct Thermocouple256; // camel case => no _

impl TemperatureSensor for Thermocouple256 {
    fn get_temp(&self) -> f64 {
        let temp: f64 = rand::random_range(128.0..256.0);
        temp
    }
}

// Explicit registration function, called from lib.rs
// Parameters:
// A name                 : "temp1" — the key you’ll later use to look it up.
// A constructor function : || Box::new(TempSensor01) — a closure that builds a new instance of this sensor.
// register_sensor() stores this name and constructor in the global registry:
//      pub static SENSOR_REGISTRY: Lazy<Mutex<HashMap<&'static str, Constructor>>> = ...
// The registry is a HashMap inside a Mutex (thread-safe).
// The key is "temp1".
// The value is a function pointer that, when called, returns Box<dyn TempSensor>.
pub fn register() {
    // See input_sensors/mod.rs
    temperature_sensor::register_sensor("Thermocouple_type_256", || Box::new(Thermocouple256));
}
