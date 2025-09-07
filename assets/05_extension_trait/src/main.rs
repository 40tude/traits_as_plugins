// main.rs
// cargo run

use std::fmt::{Display, Formatter, Result as FmtResult};

trait TempSensor {
    fn get_temp(&self) -> f64;
    fn unit(&self) -> &'static str;
}

// Trait d'extension pour l'affichage
trait SensorDisplay: TempSensor {
    fn format(&self) -> String {
        format!("{:.2} {}", self.get_temp(), self.unit())
    }
}

// Implémentation automatique pour tous les types qui implémentent TempSensor
impl<T: TempSensor> SensorDisplay for T {}

// Implémentation Display pour Box<dyn TempSensor>
impl Display for Box<dyn TempSensor> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:.2} {}", self.get_temp(), self.unit())
    }
}

// DegreeSensor implementation
struct DegreeSensor {
    value: f64,
}

impl TempSensor for DegreeSensor {
    fn get_temp(&self) -> f64 {
        self.value
    }

    fn unit(&self) -> &'static str {
        "°C"
    }
}

// FahrenheitSensor implementation
struct FahrenheitSensor {
    value: f64,
}

impl TempSensor for FahrenheitSensor {
    fn get_temp(&self) -> f64 {
        self.value
    }

    fn unit(&self) -> &'static str {
        "°F"
    }
}

fn main() {
    let sensors: Vec<Box<dyn TempSensor>> = vec![
        Box::new(DegreeSensor { value: 22.0 }),
        Box::new(FahrenheitSensor { value: 75.0 }),
        Box::new(DegreeSensor { value: 19.5 }),
    ];

    for sensor in sensors {
        println!("{}", sensor);
    }
}
