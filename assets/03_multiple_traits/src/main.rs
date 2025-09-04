// main.rs
// cargo run

// Read https://medium.com/@adamszpilewicz/implementing-and-using-traits-in-rust-step-by-step-with-custom-types-35d474bb10db

pub trait Measurable {
    // Default implementation
    fn get_temp(&self) -> f64 {
        -273.15
    }
}

pub trait Identifiable {
    // Default implementation
    fn get_id(&self) -> String {
        "NA".into()
    }
}

struct TempSensor01 {
    temp: f64,
}

impl Measurable for TempSensor01 {
    fn get_temp(&self) -> f64 {
        self.temp
    }
}

// Uses the default implementation = an empty method
impl Identifiable for TempSensor01 {}

#[allow(dead_code)]
struct TempSensor02 {
    label: String,
    temp: f64,
}

// Uses the default implementation
impl Measurable for TempSensor02 {}

impl Identifiable for TempSensor02 {
    fn get_id(&self) -> String {
        self.label.clone()
    }
}

#[allow(dead_code)]
struct TempSensor3 {
    temp: f64,
}
impl Measurable for TempSensor3 {}

// Static dispatch, generic syntax
fn inventory<T: Measurable + Identifiable>(sensor: &T) {
    println!("Sensor : {} ({} °C)", sensor.get_id(), sensor.get_temp());
}

fn main() {
    let sensor1 = TempSensor01 { temp: 100.0 };

    let sensor2 = TempSensor02 {
        label: "thermo-8086".into(),
        temp: 200.0,
    };

    inventory(&sensor1);
    inventory(&sensor2);

    // let sensor300 = TempSensor3 { temp: 300.0 };
    // inventory(&sensor3); // ! Does not compile : Identifiable is required by this bound in `inventory`
}
