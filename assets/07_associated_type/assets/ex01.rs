// Use the names of the blog post
// Temp are stored
trait TempSensor<T> {
    fn get_temp(&self) -> T;
}

// outputs temperature in Celsius as f64
struct TemSensor01 {
    temp: f64,
}

// outputs temperature in Fahrenheit as an integer (e.g., tenths of a degree)
struct TempSensor02 {
    temp: i16,
}

// Implement the trait for a Celsius-reading thermocouple
impl TempSensor<f64> for TemSensor01 {
    fn get_temp(&self) -> f64 {
        self.temp
    }
}

// Implement the trait for a Fahrenheit-reading thermocouple
impl TempSensor<i16> for TempSensor02 {
    fn get_temp(&self) -> i16 {
        // Return a Fahrenheit reading in tenths of a degree (e.g., 752 means 75.2°F).
        self.temp
    }
}

fn log_temperature<T, S>(sensor: &S)
where
    S: TempSensor<T>,
    T: std::fmt::Debug,
{
    let reading: T = sensor.get_temp();
    println!("Temperature reading: {:?}", reading);
}

fn main() {
    let sensor1 = TemSensor01 { temp: 25.0 };
    let sensor2 = TempSensor02 { temp: 777 }; // 77.7 °F

    log_temperature(&sensor1);
    log_temperature(&sensor2);
}
