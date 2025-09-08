// main.rs
// cargo run

trait TempSensor {
    fn get_temp(&self) -> f64;
    fn get_id(&self) -> String;
}

struct TempSensor01 {
    temp: f64,
    id: String,
}

impl TempSensor for TempSensor01 {
    fn get_temp(&self) -> f64 {
        self.temp
    }

    fn get_id(&self) -> String {
        "TempSensor01 - ".to_owned() + &self.id
    }
}

struct TempSensor02 {
    temp: f64,
    id: String,
}

impl TempSensor for TempSensor02 {
    fn get_temp(&self) -> f64 {
        self.temp * 9.0 / 5.0 + 32.0
    }

    fn get_id(&self) -> String {
        "TempSensor02 - ".to_owned() + &self.id
    }
}

// This trait "extends" TempSensor with extra behavior for display.
// - "trait SensorDisplay: TempSensor" means: any type that implements SensorDisplay must also implement TempSensor
// - pretty() has a default implementation (this is key) that relies on the TempSensor methods (.get_temp() and .get_id()).
// This is how extension traits add methods without impacting the original type definitions (TempSensor here)
#[allow(dead_code)]
trait SensorDisplay: TempSensor {
    fn pretty(&self) -> String {
        format!("{} {:.2} ", self.get_id(), self.get_temp(),)
    }
}

// Implement SensorDisplay for every type T that implements TempSensor
// This line blankets/generalizes all TempSensor`s with the defaulted .pretty() method
// This is possible because we own the trait (`SensorDisplay`)
// So Rust’s coherence rules let us implement it for foreign or local types as long as the trait is local
// => Any T: TempSensor automatically gets .pretty() via the extension trait
impl<T: TempSensor> SensorDisplay for T {}

fn main() {
    let sensor1 = TempSensor01 { temp: 25.0, id: "Zoubida".into() };
    let sensor2 = TempSensor02 { temp: 25.0, id: "Roberta".into() }; // 77°F

    println!("Sensor 1: {}", sensor1.pretty());
    println!("Sensor 2: {}", sensor2.pretty());
}
