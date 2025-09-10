// ex_01.rs
// cargo run --example ex_01

// Files in examples/ cannot see a private mod from src/.
// For code to be reusable from examples/, it must be exposed in /src/lib.rs.
// In examples/, we always import via use my_crate_name::... (not crate::...), unless the example is in src/bin/.

use traits_for_plugins::sensors::temp::temp_sensor::TempSensor;
// use traits_for_plugins::sensors::input::temp::temp_sensor1::my_sensor1;
use traits_for_plugins::sensors::temp::temp_sensor2::your_sensor2;

fn main() {
    // let my_sensor = my_sensor1::TempSensor01;
    let my_sensor = your_sensor2::TempSensor02;
    let my_temp = my_sensor.get_temp();
    println!("{my_temp}");
}
