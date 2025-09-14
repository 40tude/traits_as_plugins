// temperature_sensor.rs

use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

// pub trait TempSensor: Send + Sync {
pub trait TemperatureSensor {
    fn get_temp(&self) -> f64;
}

// Type alias for a constructor function returning a boxed sensor
type Constructor = fn() -> Box<dyn TemperatureSensor>;

// Global registry of sensor constructors
pub static TEMPERATURE_SENSOR_REGISTRY: Lazy<Mutex<HashMap<&'static str, Constructor>>> = Lazy::new(|| Mutex::new(HashMap::new()));

// Called by sensors (see thermocouple_128.rs for example)
// Registers a sensor constructor (e.g. `Box::new(Thermocouple)`) with a given name "Bob"
pub fn register_sensor(name: &'static str, constructor: Constructor) {
    TEMPERATURE_SENSOR_REGISTRY.lock().unwrap().insert(name, constructor);
}

// Called by binaries (main.rs, examples, tests...)
// It creates an instance of a sensor using its name
// It returns the boxed trait object
pub fn make_sensor(name: &str) -> Option<Box<dyn TemperatureSensor>> {
    TEMPERATURE_SENSOR_REGISTRY.lock().unwrap().get(name).map(|ctor| ctor())
}
