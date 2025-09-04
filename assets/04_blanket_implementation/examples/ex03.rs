// ex03
// cargo run --example ex03

// ! Does not compile

pub trait Measurable {
    fn get_temp(&self) -> f64;
}

pub trait Identifiable {
    fn get_id(&self) -> String;
}

struct TempSensor100 {
    temp: f64,
    id: String,
}

impl Measurable for TempSensor100 {
    fn get_temp(&self) -> f64 {
        self.temp
    }
}

impl Identifiable for TempSensor100 {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

// ! Yes but No
// Does not compile because of Rust's orphan/coherence rules:
// Display is a foreign trait (defined in std).
// We are not allowed to implement a foreign trait for a foreign type.
// Indeed in impl<T> Display for T where T: ..., the target type is a generic “any” type (not a local type that you own) → prohibited.
// ! The idiomatic solution is the newtype pattern: you wrap your T in a local type (that you own), then you implement Display for that wrapper. You can then offer a helper to make it easier to use.
impl<T> std::fmt::Display for T
where
    T: Measurable + Identifiable,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "id={}, temp={}", self.id, self.temp)
    }
}

fn main() {
    let sensor100 = TempSensor100 { temp: 100.0, id: "Zoubida".into() };
    println!("{}", sensor100);
}

// println!("{}", sensor100.get_id());
// Thanks to std::fmt::Display implementation
// println!("{}", sensor100);
// Thanks to the blanket implementation, this works:

// trait Printable {
//     fn print(&self);
// }

// Blanket implementation for all types that implement Display, Identifiable and Measurable

// impl<T> Printable for T where T: Display + Measurable + Identifiable { ... } is a blanket implementation.
// At compile time, Rust monomorphizes this “template” for each concrete type that satisfies these bounds.
// So if TempSensor100 implements Display, Measurable, Identifiable, it gets the Printable impl for free, and sensor100.print() compiles and works.
// The “everything you need” checks (bounds) are done at compile time → no runtime cost, no surprises.
// Two useful mini-details to keep in mind:
// This works thanks to orphan/coherence rules: you can blanket-implement your trait for “foreign” types, but not a ‘foreign’ trait for a “foreign” type.
// No overlapping implementations allowed: Rust prevents two possible implementations for the same type (ambiguities).

// impl<T> Printable for T
// where
//     // T: std::fmt::Display + Identifiable + Measurable,
//     T: Identifiable + Measurable,
// {
//     fn print(&self) {
//         // We can use the three traits together
//         // println!("Printable -> {}", self); // from Display
//         println!("  ID: {}", self.get_id()); // from Identifiable
//         println!("  Temp: {}", self.get_temp()); // from Measurable
//     }
// }

// Implement Display so it can be used in the blanket implementation
// impl std::fmt::Display for TempSensor100 {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "TempSensor100(id={}, temp={})", self.id, self.temp)
//     }
// }
