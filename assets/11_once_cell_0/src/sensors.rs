// sensors.rs
// hub file for `./sensors/` folder

pub mod temperature;

// called from main()
pub fn register() {
    temperature::register();
}
