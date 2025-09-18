// temperature_sensor.rs


// Registries are thread-safe
// But the created sensor objects aren’t guaranteed to be. 
// Let's enforcing Send + Sync on the trait objects because cross-thread usage may happen
// See TEMPERATURE_SENSOR_REGISTRY in sensors.rs 
pub trait TemperatureSensor: Send + Sync {
    fn get_temp(&self) -> f64;
}
