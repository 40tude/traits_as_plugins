// solenoid_101.rs
use crate::actuators::electric::electric_actuator::{self, ElectricActuator};

pub struct Solenoid101 {
    state: bool,
}

impl ElectricActuator for Solenoid101 {
    fn set_state(&mut self, new_state: bool) {
        self.state = new_state;
    }

    fn get_state(&self) -> bool {
        self.state
    }
}

pub fn register() {
    electric_actuator::register_actuator("Solenoid_type_101", || Box::new(Solenoid101 { state: false }));
}
