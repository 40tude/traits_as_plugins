#[derive(Debug, Clone, Copy, PartialEq)]
enum TemperatureUnit {
    Celsius,
    Fahrenheit,
    Kelvin,
}

impl TemperatureUnit {
    fn as_str(&self) -> &'static str {
        match self {
            TemperatureUnit::Celsius => "°C",
            TemperatureUnit::Fahrenheit => "°F",
            TemperatureUnit::Kelvin => "°K",
        }
    }
}

trait TempSensor {
    const UNIT: TemperatureUnit; // associated constant avec enum
    fn get_temp(&self) -> f64;
}

struct TempSensor01 {
    temp: f64,
}

impl TempSensor for TempSensor01 {
    const UNIT: TemperatureUnit = TemperatureUnit::Celsius;
    fn get_temp(&self) -> f64 {
        self.temp
    }
}

struct TempSensor02 {
    temp: f64,
}

impl TempSensor for TempSensor02 {
    const UNIT: TemperatureUnit = TemperatureUnit::Fahrenheit;
    fn get_temp(&self) -> f64 {
        self.temp * 9.0 / 5.0 + 32.0
    }
}

struct TempSensor03 {
    temp: f64,
}

impl TempSensor for TempSensor03 {
    const UNIT: TemperatureUnit = TemperatureUnit::Kelvin;
    fn get_temp(&self) -> f64 {
        self.temp + 273.15
    }
}

fn main() {
    let s1 = TempSensor01 { temp: 25.0 };
    let s2 = TempSensor02 { temp: 25.0 }; // 77 °F
    let s3 = TempSensor03 { temp: 25.0 }; // 298.15 °K

    println!("Temp 1: {:.2} {}", s1.get_temp(), TempSensor01::UNIT.as_str());
    println!("Temp 2: {:.2} {}", s2.get_temp(), TempSensor02::UNIT.as_str());
    println!("Temp 3: {:.2} {}", s3.get_temp(), TempSensor03::UNIT.as_str());
}
