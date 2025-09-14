use crate::actuators::electric::electric_actuator;
use crate::actuators::electric::electric_actuator::ElectricActuator;

pub struct Solenoid {
    state: bool,
}

impl ElectricActuator for Solenoid {
    fn set_state(&mut self, new_state: bool) {
        self.state = new_state;
    }

    fn get_state(&self) -> bool {
        self.state
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
    electric_actuator::register_actuator("Solenoid_type_101", || Box::new(Solenoid { state: false }));
}
