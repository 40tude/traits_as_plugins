// thermocouple.rs
pub mod thermocouple_128;

// Called from temperature::register()
pub fn register() {
    thermocouple_128::register();
}
