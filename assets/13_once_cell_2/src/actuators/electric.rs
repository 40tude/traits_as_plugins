// electric.rs
// hub file for the ./electric/ folder
pub mod electric_actuator; // The trait lives here
pub mod servo_motor; // Concrete sensor #1 (folder-backed)
pub mod solenoid; // Concrete sensor #2 (folder-backed)

pub fn register() {
    servo_motor::register();
    solenoid::register();
}
