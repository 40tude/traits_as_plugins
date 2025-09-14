pub mod thermocouple_128;
pub mod thermocouple_256;

pub fn register() {
    thermocouple_128::register();
    thermocouple_256::register();
}
