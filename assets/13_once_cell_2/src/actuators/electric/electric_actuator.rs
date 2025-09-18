// electric_actuator.rs
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

// pub trait ElectricActuator: Send + Sync {
pub trait ElectricActuator {
    fn set_state(&mut self, state: bool);
    fn get_state(&self) -> bool;
}

// Type alias for a constructor function returning a boxed actuator
type Constructor = fn() -> Box<dyn ElectricActuator>;

// Global registry of electrical actuators
pub static ELEC_ACTUATOR_REGISTRY: Lazy<Mutex<HashMap<&'static str, Constructor>>> = Lazy::new(|| Mutex::new(HashMap::new()));

// Called by actuators
pub fn register_actuator(name: &'static str, constructor: Constructor) {
    ELEC_ACTUATOR_REGISTRY.lock().unwrap().insert(name, constructor);
}

// Called by binaries (main.rs, examples, tests...) to creates an actuator by name
pub fn make_actuator(name: &str) -> Option<Box<dyn ElectricActuator>> {
    ELEC_ACTUATOR_REGISTRY.lock().unwrap().get(name).map(|ctor| ctor())
}
