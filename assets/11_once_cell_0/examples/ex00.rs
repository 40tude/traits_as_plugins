// ex_00
// cargo run --example ex00

use demo_registry_0::sensors;
use demo_registry_0::sensors::temperature::temperature_sensor;

fn main() {
    // Step 1 — Register all available sensors
    sensors::register();

    // Step 2 — Try creating both sensors and print their temps
    for sensor_name in ["Thermocouple_type_128", "Rtd_type_512", "temp42"] {
        match temperature_sensor::make_sensor(sensor_name) {
            Some(sensor) => {
                let temp = sensor.get_temp();
                println!("Sensor {sensor_name}: {:6.2} °C", temp);
            }
            None => {
                println!("Sensor '{sensor_name}': not found in registry!");
            }
        }
    }
}
