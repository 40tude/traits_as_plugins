// ex05
// cargo run --example ex05

// newtype pattern II
// We print with:  println!("{}", as_display(&my_sensor));

use std::fmt::{Display, Formatter, Result as FmtResult};

pub trait Measurable {
    fn get_temp(&self) -> f64;
}

pub trait Identifiable {
    fn get_id(&self) -> String;
}

struct TempSensor01 {
    temp: f64,
    id: String,
}

impl Measurable for TempSensor01 {
    fn get_temp(&self) -> f64 {
        self.temp
    }
}

impl Identifiable for TempSensor01 {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

struct TempSensor02 {
    temp: f64,
    id: String,
}

impl Measurable for TempSensor02 {
    fn get_temp(&self) -> f64 {
        self.temp
    }
}

impl Identifiable for TempSensor02 {
    fn get_id(&self) -> String {
        "TempSensor02 - ".to_owned() + &self.id
    }
}

/// Local wrapper (newtype) that you own.
/// This lets us implement foreign traits (like Display) safely.
struct AsDisplay<'a, T: Measurable + Identifiable>(&'a T);

/// Implement Display for the local wrapper, not for T directly.
/// This is allowed by the orphan rules.
impl<'a, T> Display for AsDisplay<'a, T>
where
    T: Measurable + Identifiable,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        // Use trait methods. We don't know concrete fields of T
        write!(f, "id={}, temp={}", self.0.get_id(), self.0.get_temp())
    }
}

/// Convenience function to build the wrapper without naming the type at call site.
fn as_display<T: Measurable + Identifiable>(t: &T) -> AsDisplay<'_, T> {
    AsDisplay(t)
}

fn main() {
    let sensor1 = TempSensor01 { temp: 100.0, id: "Zoubida".into() };
    let sensor2 = TempSensor02 { temp: 200.0, id: "Roberta".into() };

    // as_display looks polymorphic but no, there are 2 monomorphized implementations created at compile time
    println!("{}", as_display(&sensor1));
    println!("{}", as_display(&sensor2));
}
