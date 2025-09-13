// ex_00
// cargo run --example ex00

use demo_registry::sensors::temperature::temperature_sensor;

fn main() {
    // Step 1 — Register all available sensors
    temperature_sensor::init_sensors();

    // Step 2 — Try creating both sensors and print their temps
    for sensor_name in ["temp1", "temp2", "temp42"] {
        match temperature_sensor::make_sensor(sensor_name) {
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
