use crate::sensors::ph::ph_sensor;
use crate::sensors::ph::ph_sensor::PhSensor;

pub struct Probe; // camel case => no _

impl PhSensor for Probe {
    fn get_ph(&self) -> f64 {
        let ph: f64 = rand::random_range(7.0..8.0);
        ph
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
    ph_sensor::register_sensor("Probe_type_2048", || Box::new(Probe));
}
