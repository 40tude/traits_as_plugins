// When there is src/lib.rs, the project becomes a library crate and can also have a main binary (src/main.rs).
// In this case:
//      The code in main.rs does not see the internal modules via crate::... directly.
//      crate:: in main.rs refers to the binary itself, not to the library defined in lib.rs.
//      So, in main.rs, if you want to use code defined in lib.rs
//      you have to import it as if it were an external crate, using the crate name (the one defined in [package] name = “...” in Cargo.toml).

// use traits_for_plugins::input_sensors::{self, temp_sensor::TempSensor};
use traits_for_plugins::input_sensors::temp_sensors;
use traits_for_plugins::input_sensors::temp_sensors::TempSensor;

fn main() {
    let my_sensor: Box<dyn TempSensor> = temp_sensors::make_temp_sensor(2);
    println!("{}", my_sensor.get_temp());

    let my_sensor: Box<dyn TempSensor> = temp_sensors::make_temp_sensor(1);
    println!("{}", my_sensor.get_temp());

    // both forms works (with/without + Send + Sync)
    // Here we underline the fact that we know TempSensor is thread safe
    // But by design this is already the case
    // The line below does NOT compile if TemSensor is not + Send + Sync
    let my_sensor: Box<dyn TempSensor + Send + Sync> = temp_sensors::make_temp_sensor(2);
    println!("{}", my_sensor.get_temp());
}
