// Send : the type can be sent between threads
// Sync : the type can be shared between threads via reference (&)

// Not thread safe. Ok here
// pub trait TempSensor {

// Thread safe (overkill here?)

pub mod temp_sensor1;
pub mod temp_sensor2;

use crate::input_sensors::temp_sensors::temp_sensor1::my_sensor1;
use crate::input_sensors::temp_sensors::temp_sensor2::your_sensor2;

pub trait TempSensor: Send + Sync {
    fn get_temp(&self) -> f64; // ! Compare with 014_traits_for_plugins. Now it is &self NOT self
}

pub fn make_temp_sensor(kind: usize) -> Box<dyn TempSensor> {
    match kind {
        1 => Box::new(my_sensor1::TempSensor01),
        2 => Box::new(your_sensor2::TempSensor02),
        other => {
            // in production return a Result
            eprintln!("Unknown SENSOR_KIND='{other}', falling back to temp1.");
            Box::new(my_sensor1::TempSensor01)
        }
    }
}
