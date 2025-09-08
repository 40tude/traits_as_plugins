// main.rs
// cargo run

/// Marker trait to provide a human-readable unit label
/// Using a trait instead of an enum lets us leverage types in bounds
/// Each sensor have a unit label fixed at compile time
// Read https://medium.com/@adamszpilewicz/mastering-traits-in-rust-dynamic-dispatch-trait-objects-and-advanced-patterns-286f0ee505f4
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
    // fn get_unit_label(&self) -> &'static str {
    //     Self::Unit::LABEL
    // }
}

struct TempSensor01 {
    temp: f64,
}

impl TempSensor for TempSensor01 {
    // Associated type binds this sensor to Celsius
    type Unit = Celsius;

    fn get_temp(&self) -> f64 {
        self.temp
    }
}

struct TempSensor02 {
    temp: f64,
}

impl TempSensor for TempSensor02 {
    // Associated type binds this sensor to Fahrenheit
    type Unit = Fahrenheit;

    fn get_temp(&self) -> f64 {
        self.temp * 9.0 / 5.0 + 32.0
    }
}

impl Display for Box<dyn TempSensor> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        // Use the Unit label from the associated type.
        let unit_label = Self::Unit::LABEL;
        write!(f, "{:.2} {}", self.get_temp(), unit_label)
    }
}

fn main() {
    // Heterogeneous collection: Box<dyn TempSensor>
    let sensors: Vec<Box<dyn TempSensor>> = vec![
        Box::new(TempSensor01 { temp: 25.0 }),
        Box::new(TempSensor02 { temp: 25.0 }), // 77°F
        Box::new(TempSensor01 { temp: 42.0 }),
    ];

    for sensor in sensors {
        println!("{}", sensor);
    }
}

// /// Measurable now has an associated type `Unit`.
// /// Each implementer fixes which Unit it uses.
// trait Measurable {
//     type Unit: UnitLabel;

//     /// Return the raw temperature (assumed already expressed in `Self::Unit`).
//     fn get_temp(&self) -> f64;
// }

// trait Identifiable {
//     fn get_id(&self) -> String;
// }

// struct TempSensorC {
//     temp: f64,
//     id: String,
// }

// impl Measurable for TempSensorC {
//     // Associated type binds this sensor to Celsius
//     type Unit = Celsius;

//     fn get_temp(&self) -> f64 {
//         self.temp
//     }
// }

// impl Identifiable for TempSensorC {
//     fn get_id(&self) -> String {
//         self.id.clone()
//     }
// }

// struct TempSensorF {
//     temp: f64,
//     id: String,
// }

// impl Measurable for TempSensorF {
//     // Associated type binds this sensor to Fahrenheit
//     type Unit = Fahrenheit;

//     fn get_temp(&self) -> f64 {
//         self.temp * 9.0 / 5.0 + 32.0
//     }
// }

// impl Identifiable for TempSensorF {
//     fn get_id(&self) -> String {
//         "TempSensorF - ".to_owned() + &self.id
//     }
// }

// /// A trait we own that prints using the associated unit.
// /// The blanket impl uses the associated type in its bounds.
// trait Printable {
//     fn print(&self);
// }

// /// Blanket implementation for any T that is Identifiable + Measurable,
// /// and where the associated Unit implements UnitLabel.
// /// Note how we refer to the associated type: <T as Measurable>::Unit
// impl<T> Printable for T
// where
//     T: Identifiable + Measurable,
//     <T as Measurable>::Unit: UnitLabel,
// {
//     fn print(&self) {
//         // Use the Unit label from the associated type.
//         let unit_label = <<T as Measurable>::Unit as UnitLabel>::LABEL;

//         println!("Sensor:\n\tId = {}\n\tCurrent temp = {} {}", self.get_id(), self.get_temp(), unit_label);
//     }
// }

// fn main() {
//     let sensor1 = TempSensorC { temp: 100.0, id: "Zoubida".into() };
//     let sensor2 = TempSensorF { temp: 100.0, id: "Roberta".into() };

//     // Using the blanket impl that leverages the associated type Unit
//     sensor1.print();
//     sensor2.print();
// }
