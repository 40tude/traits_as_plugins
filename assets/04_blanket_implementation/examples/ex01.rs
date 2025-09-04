// ex01
// cargo run --example ex01

// We print with my_sensor.print()

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

struct TempSensor200 {
    temp: f64,
    id: String,
}

impl Measurable for TempSensor200 {
    fn get_temp(&self) -> f64 {
        self.temp
    }
}

impl Identifiable for TempSensor200 {
    fn get_id(&self) -> String {
        "TempSensor200 - ".to_owned() + &self.id
    }
}

trait Printable {
    fn print(&self);
}

// blanket implementation for any type implementing Identifiable and Measurable
impl<T> Printable for T
where
    T: Identifiable + Measurable,
{
    fn print(&self) {
        println!("Sensor : \n\tId = {}\n\tCurrent temp = {}", self.get_id(), self.get_temp())
    }
}

fn main() {
    let sensor100 = TempSensor100 { temp: 100.0, id: "Zoubida".into() };
    let sensor200 = TempSensor200 { temp: 200.0, id: "Roberta".into() };

    sensor100.print();
    sensor200.print();

    // ! Does not compile
    // struct Nimbus2000 {}
    // let bob = Nimbus2000 {};
    // bob.print();
}

// Implement Display so it can be used in the blanket implementation
// impl std::fmt::Display for TempSensor100 {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "TempSensor100(id={}, temp={})", self.id, self.temp)
//     }
// }

// Blanket implementation for all types that implement Display, Identifiable and Measurable

// impl<T> Printable for T where T: Display + Measurable + Identifiable { ... } is a blanket implementation.
// At compile time, Rust monomorphizes this “template” for each concrete type that satisfies these bounds.
// So if TempSensor100 implements Display, Measurable, Identifiable, it gets the Printable impl for free, and sensor100.print() compiles and works.
// The “everything you need” checks (bounds) are done at compile time → no runtime cost, no surprises.
// Two useful mini-details to keep in mind:
// This works thanks to orphan/coherence rules: you can blanket-implement your trait for “foreign” types, but not a ‘foreign’ trait for a “foreign” type.
// No overlapping implementations allowed: Rust prevents two possible implementations for the same type (ambiguities).

// println!("{}", sensor100.get_id());

// Thanks to std::fmt::Display implementation
// println!("{}", sensor100);

// Thanks to the blanket implementation, this works:

// T: std::fmt::Display + Identifiable + Measurable,
