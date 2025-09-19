// temperature.rs
// hub file for the ./temperature/ folder
pub mod rtd;
pub mod temperature_sensor; // The trait lives here
pub mod thermocouple; // Concrete sensor #1 (folder-backed) // Concrete sensor #2 (folder-backed)

pub fn register() {
    thermocouple::register();
    rtd::register();
}
