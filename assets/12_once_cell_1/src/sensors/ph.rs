// ph.rs
// hub file for the ./temperature/ folder
pub mod isfet;
pub mod ph_sensor; // The trait lives here
pub mod probe; // Concrete sensor #1 (folder-backed) // Concrete sensor #2 (folder-backed)

pub fn register() {
    isfet::register();
    probe::register();
}
