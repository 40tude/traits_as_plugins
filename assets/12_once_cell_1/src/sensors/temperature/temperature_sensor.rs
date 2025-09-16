// temperature_sensor.rs

use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

pub trait TemperatureSensor {
    fn get_temp(&self) -> f64;
}

// Type alias for a constructor function returning a boxed sensor
type Constructor = fn() -> Box<dyn TemperatureSensor>;

// Global registry of sensor constructors
pub static TEMPERATURE_SENSOR_REGISTRY: Lazy<Mutex<HashMap<&'static str, Constructor>>> = Lazy::new(|| Mutex::new(HashMap::new()));

// Called by sensors
pub fn register_sensor(name: &'static str, constructor: Constructor) {
    let mut map = TEMPERATURE_SENSOR_REGISTRY.lock().expect("TEMPERATURE_SENSOR_REGISTRY mutex poisoned");
    map.insert(name, constructor);
}

// Called by binaries (main.rs, examples, tests...) to creates a sensor by name
pub fn make_sensor(name: &str) -> Option<Box<dyn TemperatureSensor>> {
    let map = TEMPERATURE_SENSOR_REGISTRY.lock().expect("TEMPERATURE_SENSOR_REGISTRY mutex poisoned");
    map.get(name).map(|ctor| ctor())
}
