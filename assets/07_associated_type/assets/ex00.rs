// Very first version
// temp values are even hard coded

// Trait with a generic type parameter for the output

trait TemperatureSensor<T> {
    fn read_temperature(&self) -> T;
}

struct ThermocoupleC; // outputs temperature in Celsius as f64
struct ThermocoupleF; // outputs temperature in Fahrenheit as an integer (e.g., tenths of a degree)

// Implement the trait for a Celsius-reading thermocouple
impl TemperatureSensor<f64> for ThermocoupleC {
    fn read_temperature(&self) -> f64 {
        // For illustration, return a fixed value (e.g., 23.5°C).
        23.5
    }
}

// Implement the trait for a Fahrenheit-reading thermocouple
impl TemperatureSensor<i16> for ThermocoupleF {
    fn read_temperature(&self) -> i16 {
        // Return a Fahrenheit reading in tenths of a degree (e.g., 752 means 75.2°F).
        752
    }
}

fn log_temperature<T, S>(sensor: &S)
where
    S: TemperatureSensor<T>,
    T: std::fmt::Debug,
{
    let reading: T = sensor.read_temperature();
    println!("Temperature reading: {:?}", reading);
}

fn main() {
    let sensor1 = ThermocoupleC;
    let sensor2 = ThermocoupleF;

    log_temperature(&sensor1);
    log_temperature(&sensor2);
}
