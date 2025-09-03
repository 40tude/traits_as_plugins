// ex_01
// cargo run --example ex_01
//
// Files in examples/ cannot see a private mod from src/.
// For code to be reusable from examples/, it must be exposed in /src/lib.rs.
// In examples/, we always import via use my_crate_name::... (not crate::...), unless the example is in src/bin/.

// use traits_for_plugins::input_sensors::{self, temp_sensor::TempSensor};
use traits_for_plugins::input_sensors::temp_sensors;
use traits_for_plugins::input_sensors::temp_sensors::TempSensor;

fn main() {
    let my_sensor: Box<dyn TempSensor> = temp_sensors::make_temp_sensor(2);
    println!("{}", my_sensor.get_temp());

    let my_sensor: Box<dyn TempSensor> = temp_sensors::make_temp_sensor(1);
    println!("{}", my_sensor.get_temp());
}
