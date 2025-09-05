// ex04
// cargo run --example ex04

// newtype pattern I
// We print with : println!("{}", AsDisplay(&sensor1));

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

// Local wrapper (newtype) that we own.
// This lets us implement foreign traits (like Display) safely.
// Our wrapper stores a reference (&T).
// Any struct that contains a reference must name the lifetime of that reference.
// Lifetime elision works in function signatures but not in struct definitions, so the compiler forces us to add one.
// struct AsDisplay<T>( &T ); says: “I contain a borrowed T,” but we didn’t say how long that borrow must live.
// Hence E0106: missing lifetime specifier and the helpful suggestion to introduce '<a>.

// struct AsDisplay<T: Measurable + Identifiable>(&T);
struct AsDisplay<'a, T: Measurable + Identifiable>(&'a T);

/// Implement Display for the local wrapper, not for T directly.
/// This is allowed by the orphan rules.
// '_ : indicates an anonymous lifetime
impl<T> Display for AsDisplay<'_, T>
where
    T: Measurable + Identifiable,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        // Use trait methods. We don't know concrete fields of T
        write!(f, "id={}, temp={}", self.0.get_id(), self.0.get_temp())
    }
}

fn main() {
    let sensor1 = TempSensor01 { temp: 100.0, id: "Zoubida".into() };
    println!("{}", AsDisplay(&sensor1));
}
