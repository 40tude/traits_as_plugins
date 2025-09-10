// Trait with a generic type parameter for the output
trait TempSensor {
    type Output: std::fmt::Display; // associated type for the temp value
    fn get_temp(&self) -> Self::Output;
}

// outputs temperature in Celsius as f64
struct TempSensor01 {}

// Implement the trait for a Celsius-reading thermocouple
impl TempSensor for TempSensor01 {
    type Output = f64; // returns f64
    fn get_temp(&self) -> Self::Output {
        let temp: Self::Output = rand::random_range(10.0..35.0);
        temp
    }
}

// outputs temperature in Fahrenheit as an integer
struct TempSensor02 {}

// Implement the trait for a Fahrenheit-reading thermocouple
impl TempSensor for TempSensor02 {
    type Output = i16; // returns i16

    fn get_temp(&self) -> Self::Output {
        // Return a Fahrenheit reading in tenths of a degree (752 means 75.2°F).
        let temp: Self::Output = rand::random_range(500..950);
        temp
    }
}

fn log_temperature<S: TempSensor>(sensor: &S) {
    let reading: S::Output = sensor.get_temp();
    println!("Temperature reading: {}", reading);
}

fn main() {
    let sensor1 = TempSensor01 {};
    let sensor2 = TempSensor02 {};

    log_temperature(&sensor1);
    log_temperature(&sensor2);
}

// Similar but using supertrait
// trait TempSensor
// where
//     Self::Output: std::fmt::Display,
// {
//     type Output;
//     fn get_temp(&self) -> Self::Output;
// }
