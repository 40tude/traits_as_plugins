// main.rs
// cargo add rand
// cargo add once_cell

use demo_registry_1_bis::sensors;
use demo_registry_1_bis::sensors::PH_SENSOR_REGISTRY;
use demo_registry_1_bis::sensors::TEMPERATURE_SENSOR_REGISTRY;

fn main() {
    sensors::register();

    let sensor_name: String = "Thermocouple_type_128".into();
    let thermo_01 = TEMPERATURE_SENSOR_REGISTRY.make_sensor(&sensor_name).expect("Unknown sensor");
    println!("Thermocouple 01: {:6.2}", thermo_01.get_temp());

    let thermo_02 = TEMPERATURE_SENSOR_REGISTRY.make_sensor("Thermocouple_type_256").expect("Unknown sensor");
    println!("Thermocouple 02: {:6.2}", thermo_02.get_temp());

    let thermo_03 = TEMPERATURE_SENSOR_REGISTRY.make_sensor("Thermocouple_type_256").expect("Unknown sensor");
    println!("Thermocouple 03: {:6.2}", thermo_03.get_temp());

    let rtd_01 = TEMPERATURE_SENSOR_REGISTRY.make_sensor("Rtd_type_512").expect("Unknown sensor");
    println!("RTD 01         : {:6.2}", rtd_01.get_temp());

    let isfet_01 = PH_SENSOR_REGISTRY.make_sensor("IsFET_type_1024").expect("Unknown sensor");
    println!("IsFET_01       : {:6.2}", isfet_01.get_ph());

    let probe_42 = PH_SENSOR_REGISTRY.make_sensor("Probe_type_2048").expect("Unknown sensor");
    println!("Probe_42       : {:6.2}", probe_42.get_ph());
}
