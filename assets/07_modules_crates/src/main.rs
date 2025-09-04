// main.rs
// cargo run

// When there is src/lib.rs, the project becomes a library crate and can also have a main binary (src/main.rs).
// But in this case:
//      The code in main.rs does not see the internal modules via crate::... directly.
//      crate:: in main.rs refers to the binary itself, not to the library defined in lib.rs.
//      So, in main.rs, if you want to use code defined in lib.rs
//      you have to import it as if it were an external crate, using the crate name (the one defined in [package] name = “...” in Cargo.toml).

use traits_for_plugins::sensors::temp::temp_sensor::TempSensor;
use traits_for_plugins::sensors::temp::temp_sensor1::my_sensor1; // temp_sensor2

fn main() {
    let my_sensor = my_sensor1::TempSensor01;
    let my_temp = my_sensor.get_temp();
    println!("{my_temp}");
}
