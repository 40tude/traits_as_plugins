// cargo add rand
// Read : https://doc.rust-lang.org/book/ch20-02-advanced-traits.html#:~:text=The%20difference%20is%20that%20when,we%20want%20to%20use

trait TempSensor<T> {
    fn get_temp(&self) -> T;
}

// outputs temperature in Celsius as f64
struct TempSensor01 {}

// Implement the trait for a Celsius-reading thermocouple
impl TempSensor<f64> for TempSensor01 {
    fn get_temp(&self) -> f64 {
        let temp: f64 = rand::random_range(10.0..35.0);
        temp
    }
}

// outputs temperature in Fahrenheit as an integer
struct TempSensor02 {}

// Implement the trait for a Fahrenheit-reading thermocouple
impl TempSensor<i16> for TempSensor02 {
    fn get_temp(&self) -> i16 {
        // Return a Fahrenheit reading in tenths of a degree (752 means 75.2°F).
        let temp: i16 = rand::random_range(500..950);
        temp
    }
}

// Verbose Trait Bounds
// Whenever we use TempSensor in a function, we must specify the type parameter and add trait bounds for it.
// For example, a function to log a temperature might need to be generic over both the sensor type and the return type
// This function signature is quite verbose
//      - We had to introduce a placeholder type T
//      - And a trait bound S: TemperatureSensor<T> just to use the sensor.
// If the trait had multiple type parameters, the complexity would grow even more.

// Multiple Implementations Confusion:
// Because the trait is generic, it's theoretically possible for a single sensor type to implement TempSensor for different output types.
// One could implement TempSensor<f64> and TempSensor<i16> for the same struct (if it made any sense).
// This means the compiler must allow the possibility of multiple trait impls for one type, which can lead to ambiguity.
// As the Rust documentation notes, “when a trait has a generic parameter, it can be implemented for a type multiple times, changing the concrete types of the generic type parameters each time”
// In practice, we should avoid doing that for a sensor, but the trait’s design permits it, so using the trait might require additional type annotation to disambiguate which implementation you need.

// In short, the fully generic trait solution gets the job done but is cumbersome.
// The code using the trait becomes less readable due to all the angle brackets and type parameters.
// This is where associated types come in to save the day.
fn log_temperature<T, S>(sensor: &S)
where
    S: TempSensor<T>,
    T: std::fmt::Display,
{
    let reading: T = sensor.get_temp();
    println!("Temperature reading: {}", reading);
}

fn main() {
    let sensor1 = TempSensor01 {};
    let sensor2 = TempSensor02 {};

    log_temperature(&sensor1);
    log_temperature(&sensor2);
}
