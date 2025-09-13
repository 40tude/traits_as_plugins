// main.rs
use traits_for_plugins::sensors::temperature::temperature_sensor::{self, TempSensor};

fn main() {
    let mut sensors: Vec<Box<dyn TempSensor>> = Vec::new();
    sensors.push(temperature_sensor::make_sensor(2));
    sensors.push(temperature_sensor::make_sensor(1));
    sensors.push(temperature_sensor::make_sensor(2));

    for s in sensors {
        println!("{}", s.get_temp());
    }
}

// let my_sensor: Box<dyn TempSensor> = temperature_sensor::make_sensor(2);
// println!("{}", my_sensor.get_temp());

// let my_sensor: Box<dyn TempSensor> = temperature_sensor::make_sensor(1);
// println!("{}", my_sensor.get_temp());

// let my_sensor: Box<dyn TempSensor> = temperature_sensor::make_sensor(2);
// println!("{}", my_sensor.get_temp());

// let sensors = vec![
//     temperature_sensor::make_sensor(2),
//     temperature_sensor::make_sensor(1),
//     temperature_sensor::make_sensor(2),
// ];

// In a multi threaded context we must use
// let my_sensor: Box<dyn TempSensor + Send + Sync> = temperature_sensor::make_sensor(2);
// The line above does NOT compile if TempSensor is NOT + Send + Sync (see src/sensors/temperature/temperature_sensor.rs)
