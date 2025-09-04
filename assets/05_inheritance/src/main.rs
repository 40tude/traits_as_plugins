// main.rs.05
// cargo run

// Read https://medium.com/@adamszpilewicz/mastering-traits-in-rust-dynamic-dispatch-trait-objects-and-advanced-patterns-286f0ee505f4

use std::fmt::{Display, Formatter, Result as FmtResult};

// Trait that all sensors must implement
#[allow(dead_code)]
trait TempSensor: Display {
    fn get_temp(&self) -> f64;
}

// DegreeSensor implementation
struct DegreeSensor {
    value: f64,
}

impl TempSensor for DegreeSensor {
    fn get_temp(&self) -> f64 {
        self.value
    }
}

impl Display for DegreeSensor {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:.2} °C", self.value)
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
}

impl Display for FahrenheitSensor {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:.2} °F", self.value)
    }
}

fn main() {
    // Heterogeneous collection: Box<dyn TempSensor>
    let sensors: Vec<Box<dyn TempSensor>> = vec![
        Box::new(DegreeSensor { value: 22.0 }),
        Box::new(FahrenheitSensor { value: 75.0 }),
        Box::new(DegreeSensor { value: 19.5 }),
    ];

    for sensor in sensors {
        // Works because TempSensor requires Display
        println!("{:}", sensor);
    }
}
