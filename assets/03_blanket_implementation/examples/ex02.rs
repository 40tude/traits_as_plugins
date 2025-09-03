// ex02
// cargo run --example ex02

// We print with println!("{}", sensor100.pretty());

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

trait PrettyFmt {
    fn pretty(&self) -> String;
}

// Blanket impl for any T: Identifiable + Measurable
impl<T> PrettyFmt for T
where
    T: Identifiable + Measurable,
{
    fn pretty(&self) -> String {
        format!("Sensor : \n\tId = {}\n\tCurrent temp = {}", self.get_id(), self.get_temp())
    }
}

fn main() {
    let sensor100 = TempSensor100 { temp: 100.0, id: "Zoubida".into() };
    let sensor200 = TempSensor200 { temp: 200.0, id: "Roberta".into() };

    println!("{}", sensor100.pretty());
    println!("{}", sensor200.pretty());
}
