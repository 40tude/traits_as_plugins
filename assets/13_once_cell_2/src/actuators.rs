// actuators.rs
// hub file for `./actuators/` folder

pub mod electric;

pub fn register() {
    electric::register();
}
