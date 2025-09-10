trait TempSensor {
    const UNIT: &'static str; // associated constant
    fn get_temp(&self) -> f64;
}

struct TempSensor01 {
    temp: f64,
}

impl TempSensor for TempSensor01 {
    const UNIT: &'static str = "°C";
    fn get_temp(&self) -> f64 {
        self.temp
    }
}

struct TempSensor02 {
    temp: f64,
}
impl TempSensor for TempSensor02 {
    const UNIT: &'static str = "°F";
    fn get_temp(&self) -> f64 {
        self.temp * 9.0 / 5.0 + 32.0
    }
}

fn main() {
    let s1 = TempSensor01 { temp: 25.0 };
    let s2 = TempSensor02 { temp: 25.0 }; // 77 °F

    println!("Temp 1: {} {}", s1.get_temp(), TempSensor01::UNIT);
    println!("Temp 2: {} {}", s2.get_temp(), TempSensor02::UNIT);
}
