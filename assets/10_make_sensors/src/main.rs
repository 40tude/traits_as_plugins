// main.rs
// use traits_for_plugins::sensors::temperature::temperature_sensor;
// use traits_for_plugins::sensors::temperature::temperature_sensor::TempSensor;
use traits_for_plugins::sensors::temperature::temperature_sensor::{self, TempSensor};

fn main() {
    let my_sensor: Box<dyn TempSensor> = temperature_sensor::make_sensor(2);
    println!("{}", my_sensor.get_temp());

    let my_sensor: Box<dyn TempSensor> = temperature_sensor::make_sensor(1);
    println!("{}", my_sensor.get_temp());

    // both forms works (with/without + Send + Sync)
    // Here we underline the fact that we know TempSensor is thread safe
    // But by design this is already the case
    // The line below does NOT compile if TempSensor is NOT + Send + Sync
    let my_sensor: Box<dyn TempSensor + Send + Sync> = temperature_sensor::make_sensor(2);
    println!("{}", my_sensor.get_temp());
}
