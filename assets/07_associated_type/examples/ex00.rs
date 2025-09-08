trait UnitLabel {
    const LABEL: &'static str;
}

struct Celsius;
struct Fahrenheit;

impl UnitLabel for Celsius {
    const LABEL: &'static str = "°C";
}

impl UnitLabel for Fahrenheit {
    const LABEL: &'static str = "°F";
}

trait TempSensor {
    type Unit: UnitLabel;
    fn get_temp(&self) -> f64;
}

struct TempSensor01 {
    temp: f64,
}

impl TempSensor for TempSensor01 {
    type Unit = Celsius;
    fn get_temp(&self) -> f64 {
        self.temp
    }
}

struct TempSensor02 {
    temp: f64,
}

impl TempSensor for TempSensor02 {
    type Unit = Fahrenheit;
    fn get_temp(&self) -> f64 {
        self.temp * 9.0 / 5.0 + 32.0
    }
}

trait Printable {
    fn print(&self);
}

impl<T> Printable for T
where
    T: TempSensor,
    <T as TempSensor>::Unit: UnitLabel,
{
    fn print(&self) {
        let unit_label = <<T as TempSensor>::Unit as UnitLabel>::LABEL;
        println!("Current temp = {} {}", self.get_temp(), unit_label);
    }
}

fn main() {
    let sensor1 = TempSensor01 { temp: 25.0 };
    let sensor2 = TempSensor02 { temp: 25.0 }; // 77°F

    // Using the blanket impl that leverages the associated type Unit
    sensor1.print();
    sensor2.print();
}
