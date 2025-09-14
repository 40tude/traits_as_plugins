// main.rs

// cargo add rand
// cargo add once_cell

use demo_registry_1::sensors;
use demo_registry_1::sensors::ph::ph_sensor;
use demo_registry_1::sensors::temperature::temperature_sensor;

fn main() {
    // Register all available sensors and actuators
    // It calls each module’s register() function.
    sensors::register();

    // The global SENSOR_REGISTRY now contains:
    // {
    //     "temp1" => (constructor for TempSensor01),
    //     "temp2" => (constructor for TempSensor02)
    // }

    // see input_sensor/mod.rs
    // make_sensor() does :
    //      Lock the registry’s Mutex to safely access the HashMap.
    //      Look up the key "temp1".
    //      If found, take the stored constructor function and call it.
    //      This executes || Box::new(TempSensor01) — building a new instance of TempSensor01 and boxing it as a Box<dyn TempSensor>.
    //      Return that boxed trait object.
    let thermo_01 = temperature_sensor::make_sensor("Thermocouple_type_128").expect("Unknown sensor");
    let thermo_02 = temperature_sensor::make_sensor("Thermocouple_type_256").expect("Unknown sensor");

    // sensor a Box<dyn TempSensor> is pointing to a TempSensor01 instance.
    // Because TempSensor is in scope, we can call get_temp() directly
    // This uses dynamic dispatch at runtime (fat pointers, vtable lookup).
    // The method returns a temperature
    println!("\nSensors:");
    println!("Thermocouple 01: {:6.2}", thermo_01.get_temp());
    println!("Thermocouple 02: {:6.2}", thermo_02.get_temp());

    let rtd_01 = temperature_sensor::make_sensor("Rtd_type_512").expect("Unknown sensor");
    println!("RTD 01         : {:6.2}", rtd_01.get_temp());

    let isfet_01 = ph_sensor::make_sensor("IsFET_type_1024").expect("Unknown sensor");
    println!("IsFET_01       : {:6.2}", isfet_01.get_ph());

    let probe_42 = ph_sensor::make_sensor("Probe_type_2048").expect("Unknown sensor");
    println!("Probe_42       : {:6.2}", probe_42.get_ph());
}
