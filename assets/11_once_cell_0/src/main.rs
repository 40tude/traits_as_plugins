// main.rs

// cargo add rand
// cargo add once_cell

use demo_registry_0::sensors;
use demo_registry_0::sensors::temperature::temperature_sensor;

fn main() {
    // Register all sensors available
    // It calls each module’s register() function.
    sensors::register();

    // At this point the global TEMPERATURE_SENSOR_REGISTRY now contains:
    // {
    //     "Thermocouple_type_128" => (constructor for Thermocouple),
    //     "Rtd_type_512" => (constructor for Rtd)
    // }

    // At this point, none of the sensors have been initialized yet (due to lazy initialization).

    // See ./sensors/temperature/temperature_sensor.rs
    // make_sensor() does :
    //      Lock the registry’s Mutex to safely access the HashMap.
    //      Look for the key "Thermocouple_type_128".
    //      If found, take the stored constructor function and call it.
    //      This executes Box::new(Thermocouple) — building a new instance of Thermocouple and boxing it as a Box<dyn TemperatureSensor>.
    //      Returns the boxed trait object.
    let thermo_01 = temperature_sensor::make_sensor("Thermocouple_type_128").expect("Unknown sensor");

    // thermo_01 is a Box<dyn TemperatureSensor> pointing to a Thermocouple instance.
    // Because TemperatureSensor trait is in scope, we can call .get_temp() directly
    // This uses dynamic dispatch at runtime (fat pointers, vtable lookup).
    // The method returns a temperature
    println!("Thermocouple 01: {:6.2}", thermo_01.get_temp());

    let rtd_01 = temperature_sensor::make_sensor("Rtd_type_512").expect("Unknown sensor");
    println!("RTD 01         : {:6.2}", rtd_01.get_temp());
}
