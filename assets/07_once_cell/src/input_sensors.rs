use crate::input_sensors;

pub mod temp_sensors;

/// Called by binaries (main.rs, examples, tests...) to register all sensors
pub fn init_sensors() {
    // Explicitly register each available sensor
    //
    input_sensors::temp_sensors::temp_sensor1::my_sensor1::register();
    input_sensors::temp_sensors::temp_sensor2::your_sensor2::register();
}
