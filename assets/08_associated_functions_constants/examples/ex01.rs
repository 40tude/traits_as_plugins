trait TempSensor {
    const UNIT: &'static str;

    fn get_temp(&self) -> f64;

    // Associated function (no self)
    fn new_set_to_zero() -> Self
    where
        Self: Sized;
}

struct TempSensor01 {
    temp: f64,
}

impl TempSensor for TempSensor01 {
    const UNIT: &'static str = "°C";

    fn get_temp(&self) -> f64 {
        self.temp
    }

    fn new_set_to_zero() -> Self {
        TempSensor01 { temp: 0.0 }
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

    fn new_set_to_zero() -> Self {
        TempSensor02 { temp: 0.0 }
    }
}

fn main() {
    let s1 = TempSensor01::new_set_to_zero();
    let s2 = TempSensor02::new_set_to_zero();

    println!("Factory sensor 1: {} {}", s1.get_temp(), TempSensor01::UNIT);
    println!("Factory sensor 2: {} {}", s2.get_temp(), TempSensor02::UNIT);
}
