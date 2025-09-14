// sensors.rs
// hub file for `./sensors/` folder

pub mod ph;
pub mod temperature;

pub fn register() {
    temperature::register();
    ph::register();
}
