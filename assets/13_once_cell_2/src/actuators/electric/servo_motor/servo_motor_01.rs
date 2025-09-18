// servo_motor_01.rs
use crate::actuators::electric::electric_actuator::{self, ElectricActuator};

pub struct ServoMotor01 {
    state: bool,
}

impl ElectricActuator for ServoMotor01 {
    fn set_state(&mut self, new_state: bool) {
        self.state = new_state;
    }

    fn get_state(&self) -> bool {
        self.state
    }
}

pub fn register() {
    electric_actuator::register_actuator("Servo_Motor_type_01", || Box::new(ServoMotor01 { state: false }));
}
