// temperature_sensor.rs
use crate::sensors::temperature::temperature_sensor1::my_sensor1;
use crate::sensors::temperature::temperature_sensor2::your_sensor2;

// pub mod temp_sensors;
// use crate::input_sensors;
// use crate::sensors::temperature::temperature_sensor;

use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

// use crate::input_sensors::temp_sensors::TempSensor;

/// Type alias for a constructor function returning a boxed sensor
type Constructor = fn() -> Box<dyn TempSensor>;

/// Global registry of sensor constructors
pub static SENSOR_REGISTRY: Lazy<Mutex<HashMap<&'static str, Constructor>>> = Lazy::new(|| Mutex::new(HashMap::new()));

/// Registers a sensor constructor with a given name
pub fn register_sensor(name: &'static str, constructor: Constructor) {
    SENSOR_REGISTRY.lock().unwrap().insert(name, constructor);
}

/// Creates a sensor by name
pub fn make_sensor(name: &str) -> Option<Box<dyn TempSensor>> {
    SENSOR_REGISTRY.lock().unwrap().get(name).map(|ctor| ctor())
}

pub trait TempSensor: Send + Sync {
    fn get_temp(&self) -> f64;
}

/// Called by binaries (main.rs, examples, tests...) to register all sensors
pub fn init_sensors() {
    // Explicitly register each available sensor
    //
    // input_sensors::temp_sensors::temp_sensor1::my_sensor1::register();
    // input_sensors::temp_sensors::temp_sensor2::your_sensor2::register();
    my_sensor1::register();
    your_sensor2::register();
}
