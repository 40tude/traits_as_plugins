use crate::input_sensors::temp_sensors::TempSensor;
use crate::input_sensors::temp_sensors::register_sensor;

pub struct TempSensor01;

impl TempSensor for TempSensor01 {
    fn get_temp(&self) -> f64 {
        142.0
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
    register_sensor("temp1", || Box::new(TempSensor01));
}
