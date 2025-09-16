// main.rs
// cargo add rand
// cargo add once_cell
use demo_registry_1::sensors;
use demo_registry_1::sensors::ph::ph_sensor;
use demo_registry_1::sensors::temperature::temperature_sensor;

fn main() {
    sensors::register();

    let thermo_01 = temperature_sensor::make_sensor("Thermocouple_type_128").expect("Unknown sensor");
    let thermo_02 = temperature_sensor::make_sensor("Thermocouple_type_256").expect("Unknown sensor");
    let thermo_03 = temperature_sensor::make_sensor("Thermocouple_type_256").expect("Unknown sensor");

    println!("\nSensors:");
    println!("Thermocouple 01: {:6.2}", thermo_01.get_temp());
    println!("Thermocouple 02: {:6.2}", thermo_02.get_temp());
    println!("Thermocouple 03: {:6.2}", thermo_03.get_temp());

    let rtd_01 = temperature_sensor::make_sensor("Rtd_type_512").expect("Unknown sensor");
    println!("RTD 01         : {:6.2}", rtd_01.get_temp());

    let isfet_01 = ph_sensor::make_sensor("IsFET_type_1024").expect("Unknown sensor");
    println!("IsFET_01       : {:6.2}", isfet_01.get_ph());

    let probe_42 = ph_sensor::make_sensor("Probe_type_2048").expect("Unknown sensor");
    println!("Probe_42       : {:6.2}", probe_42.get_ph());
}
