// temperature_sensor.rs
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

// pub trait ElectricActuator: Send + Sync {
pub trait ElectricActuator {
    fn set_state(&mut self, state: bool);
    fn get_state(&self) -> bool;
}

// Type alias for a constructor function returning a boxed sensor
type Constructor = fn() -> Box<dyn ElectricActuator>;

// Global registry of sensor constructors
pub static SENSOR_REGISTRY: Lazy<Mutex<HashMap<&'static str, Constructor>>> = Lazy::new(|| Mutex::new(HashMap::new()));

// Called by sensors (see my_sensor1.rs for example)
// Registers a sensor constructor with a given name
pub fn register_actuator(name: &'static str, constructor: Constructor) {
    SENSOR_REGISTRY.lock().unwrap().insert(name, constructor);
}

// Called by binaries (main.rs, examples, tests...) to creates a sensor by name
pub fn make_actuator(name: &str) -> Option<Box<dyn ElectricActuator>> {
    SENSOR_REGISTRY.lock().unwrap().get(name).map(|ctor| ctor())
}
