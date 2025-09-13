// main.rs
// cargo add once_cell

use demo_registry::sensors::temperature::temperature_sensor;

fn main() {
    // Register all available sensors
    // It calls each module’s register() function.
    // see lib.rs
    temperature_sensor::init_sensors();

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
    let sensor = temperature_sensor::make_sensor("temp1").expect("Unknown sensor");

    // sensor a Box<dyn TempSensor> is pointing to a TempSensor01 instance.
    // Because TempSensor is in scope, we can call get_temp() directly
    // This uses dynamic dispatch at runtime (fat pointers, vtable lookup).
    // The method returns a temperature
    println!("Sensor1 temp: {}", sensor.get_temp());

    let sensor = temperature_sensor::make_sensor("temp2").expect("Unknown sensor");
    println!("Sensor2 temp: {}", sensor.get_temp());
}
