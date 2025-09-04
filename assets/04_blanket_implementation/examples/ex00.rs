// ex00
// cargo run --example ex00

// Implement std::fmt::Display() but only for TempSensor100 datatype
// We can write println!("{}", my_sensor);
// Read https://medium.com/@adamszpilewicz/implementing-and-using-traits-in-rust-step-by-step-with-custom-types-35d474bb10db

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

// Implement Display so it can be used with the printl!() macro
impl std::fmt::Display for TempSensor100 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Sensor type : TempSensor100\n\tId = {}\n\tCurrent temp = {}", self.id, self.temp)
    }
}

fn main() {
    let sensor100 = TempSensor100 { temp: 100.0, id: "Zoubida".into() };
    println!("{}", sensor100);
}
