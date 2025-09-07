// main.rs
// cargo run

// Read https://medium.com/@adamszpilewicz/mastering-traits-in-rust-dynamic-dispatch-trait-objects-and-advanced-patterns-286f0ee505f4

use std::fmt::{Display, Formatter, Result as FmtResult};

// TempSensor inherits from Display.
// Any type that wants to implement TempSensor must also implement Display.
// The Rust compiler does not imply this automatically, so it is up to us to implement Display on our types in addition to TempSensor.
#[allow(dead_code)]
trait TempSensor: Display {
    fn get_temp(&self) -> f64;
}

// TempSensor01 implementation
struct TempSensor01 {
    temp: f64,
}

impl TempSensor for TempSensor01 {
    fn get_temp(&self) -> f64 {
        self.temp
    }
}

impl Display for TempSensor01 {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:.2} °C", self.temp)
    }
}

// TempSensor02 implementation
struct TempSensor02 {
    temp: f64,
}

impl TempSensor for TempSensor02 {
    fn get_temp(&self) -> f64 {
        self.temp * 9.0 / 5.0 + 32.0
    }
}

impl Display for TempSensor02 {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:.2} °F", self.get_temp())
    }
}

fn main() {
    // Heterogeneous collection: Box<dyn TempSensor>
    let sensors: Vec<Box<dyn TempSensor>> = vec![
        Box::new(TempSensor01 { temp: 25.0 }),
        Box::new(TempSensor02 { temp: 25.0 }), // 77 °F
        Box::new(TempSensor01 { temp: 42.0 }),
    ];

    for sensor in sensors {
        // Works because TempSensor requires Display
        println!("{}", sensor);
    }
}
