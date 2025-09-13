// temperature_sensor.rs
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

use crate::sensors::temperature::temperature_sensor1::my_sensor1;
use crate::sensors::temperature::temperature_sensor2::your_sensor2;

// pub trait TempSensor: Send + Sync {
pub trait TempSensor {
    fn get_temp(&self) -> f64;
}

// Type alias for a constructor function returning a boxed sensor
type Constructor = fn() -> Box<dyn TempSensor>;

// Global registry of sensor constructors
pub static SENSOR_REGISTRY: Lazy<Mutex<HashMap<&'static str, Constructor>>> = Lazy::new(|| Mutex::new(HashMap::new()));

// Called by sensors (see my_sensor1.rs for example)
// Registers a sensor constructor with a given name
pub fn register_sensor(name: &'static str, constructor: Constructor) {
    SENSOR_REGISTRY.lock().unwrap().insert(name, constructor);
}

// Called by binaries (main.rs, examples, tests...) to creates a sensor by name
pub fn make_sensor(name: &str) -> Option<Box<dyn TempSensor>> {
    SENSOR_REGISTRY.lock().unwrap().get(name).map(|ctor| ctor())
}

// Called by binaries (main.rs, examples, tests...) to register all sensors
// Explicitly register each available sensor
pub fn init_sensors() {
    my_sensor1::register();
    your_sensor2::register();
}
