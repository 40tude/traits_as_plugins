// probe_2048.rs
use crate::sensors::ph::ph_sensor::PhSensor;
use crate::sensors::PH_SENSOR_REGISTRY;

pub struct Probe2048; // camel case => no _

impl PhSensor for Probe2048 {
    fn get_ph(&self) -> f64 {
        let ph: f64 = rand::random_range(7.0..8.0);
        ph
    }
}

// Explicit registration function
pub fn register() {
    PH_SENSOR_REGISTRY.register_sensor("Probe_type_2048", || Box::new(Probe2048));
}
