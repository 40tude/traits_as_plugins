// temperature.rs
// hub file for the ./temperature/ folder

pub mod rtd; // Concrete sensor
pub mod temperature_sensor; // The trait lives here
pub mod thermocouple; // Concrete sensor

// Called from sensors::register()
pub fn register() {
    thermocouple::register();
    rtd::register();
}
