// temperature_sensor.rs
use crate::sensors::temperature::temperature_sensor1::my_sensor1;
use crate::sensors::temperature::temperature_sensor2::your_sensor2;

pub trait TempSensor {
    fn get_temp(&self) -> f64;
}

pub fn make_sensor(kind: usize) -> Box<dyn TempSensor> {
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

// Send : the type can be sent between threads
// Sync : the type can be shared between threads via reference (&)
// We would have to write
// pub trait TempSensor: Send + Sync {
// Here we are single threaded so is not needed
