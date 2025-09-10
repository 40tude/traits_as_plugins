use std::fmt::{Display, Formatter, Result as FmtResult};

trait UnitLabel {
    const LABEL: &'static str;
}

struct Celsius;
struct Fahrenheit;

impl UnitLabel for Celsius {
    const LABEL: &'static str = "°C";
}

impl UnitLabel for Fahrenheit {
    const LABEL: &'static str = "°F";
}

trait TempSensor {
    type Unit: UnitLabel;
    fn get_temp(&self) -> f64;
    fn get_unit_label(&self) -> &'static str {
        Self::Unit::LABEL
    }
}

struct TempSensor01 {
    temp: f64,
}

impl TempSensor for TempSensor01 {
    type Unit = Celsius;

    fn get_temp(&self) -> f64 {
        self.temp
    }
}

struct TempSensor02 {
    temp: f64, // Stocké en Celsius mais affiché en Fahrenheit
}

impl TempSensor for TempSensor02 {
    type Unit = Fahrenheit;

    fn get_temp(&self) -> f64 {
        self.temp * 9.0 / 5.0 + 32.0 // Conversion Celsius → Fahrenheit
    }
}

impl Display for dyn TempSensor {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        // Utilisation de la méthode get_unit_label() pour obtenir le label
        write!(f, "{:.2} {}", self.get_temp(), self.get_unit_label())
    }
}

fn main() {
    let sensors: Vec<Box<dyn TempSensor>> = vec![
        Box::new(TempSensor01 { temp: 25.0 }),
        Box::new(TempSensor02 { temp: 25.0 }), // 25°C → 77°F
        Box::new(TempSensor01 { temp: 42.0 }),
    ];

    for sensor in sensors {
        println!("{}", sensor);
    }
}
