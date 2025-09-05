// ex01
// cargo run --example ex01

// We print with : my_sensor.print()
// .print() prints to the console

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

trait Printable {
    fn print(&self);
}

// Blanket implementation for any type implementing Identifiable and Measurable
// At compile time, Rust monomorphizes this “template” for each concrete type that satisfies these bounds.
// So if TempSensor01 implements Identifiable and Measurable, it gets the Printable impl for "free", and sensor1.print() compiles and works.
// The checks (bounds) are done at compile time → no runtime cost, no surprises.
// Keep in mind:
//      This works thanks to orphan/coherence rules: we can blanket-implement our trait for “foreign” types, but not a 'foreign' trait for a 'foreign' type.
//      No overlapping implementations allowed: Rust prevents two possible implementations for the same type (ambiguities).
impl<T> Printable for T
where
    T: Identifiable + Measurable,
{
    fn print(&self) {
        println!("Sensor : \n\tId = {}\n\tCurrent temp = {}", self.get_id(), self.get_temp())
    }
}

fn main() {
    let sensor1 = TempSensor01 { temp: 100.0, id: "Zoubida".into() };
    let sensor2 = TempSensor02 { temp: 200.0, id: "Roberta".into() };

    sensor1.print();
    sensor2.print();

    // ! Does not compile.
    // Nimbus2000 does not have the Identifiable nor the Measurable trait => Does not get the Printable trait
    struct Nimbus2000 {}
    let bob = Nimbus2000 {};
    bob.print();
}
