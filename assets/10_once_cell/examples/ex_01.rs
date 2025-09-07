// ex_01
// cargo run --example ex_01

// Files in examples/ cannot see a private mod from src/.
// For code to be reusable from examples/, it must be exposed in /src/lib.rs.
// In examples/, we always use my_crate_name::... (not crate::...) (where my_crate_name is defined in [package] name = “...” in Cargo.toml).

use demo_registry::input_sensors::{init_sensors, temp_sensors::make_sensor};

fn main() {
    // Step 1 — Register all available sensors
    init_sensors();

    // Step 2 — Try creating both sensors and print their temps
    for sensor_name in ["temp1", "temp2", "temp42"] {
        match make_sensor(sensor_name) {
            Some(sensor) => {
                let temp = sensor.get_temp();
                println!("Sensor '{sensor_name}' reports temperature: {temp}");
            }
            None => {
                println!("Sensor '{sensor_name}' not found in registry!");
            }
        }
    }
}
