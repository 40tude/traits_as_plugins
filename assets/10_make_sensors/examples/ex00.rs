// ex00.rs
// cargo run --example ex00

use traits_for_plugins::sensors::temperature::temperature_sensor::{self, TempSensor};

fn main() {
    let my_sensor: Box<dyn TempSensor> = temperature_sensor::make_sensor(2);
    println!("{}", my_sensor.get_temp());

    let my_sensor: Box<dyn TempSensor> = temperature_sensor::make_sensor(1);
    println!("{}", my_sensor.get_temp());
}
