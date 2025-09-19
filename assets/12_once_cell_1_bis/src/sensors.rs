// sensors.rs
// hub file for `./sensors/` folder

pub mod ph;
pub mod sensor_registry;
pub mod temperature;

use crate::sensors::ph::ph_sensor::PhSensor;
use crate::sensors::sensor_registry::Registry;
use crate::sensors::temperature::temperature_sensor::TemperatureSensor;
use once_cell::sync::Lazy;

// One global registry per kind of sensor (temperature, pH...)
// Multi thread is supported with + Send + Sync
// See temperature_sensors.rs and ph_sensors.rs
pub static TEMPERATURE_SENSOR_REGISTRY: Lazy<Registry<dyn TemperatureSensor + Send + Sync>> = Lazy::new(Registry::default);
pub static PH_SENSOR_REGISTRY: Lazy<Registry<dyn PhSensor + Send + Sync>> = Lazy::new(Registry::default);

pub fn register() {
    temperature::register();
    ph::register();
}
