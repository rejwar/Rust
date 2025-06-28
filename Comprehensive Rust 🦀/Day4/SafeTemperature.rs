pub struct Celsius(f64);

impl Celsius {
    pub fn new(temp: f64) -> Option<Self> {
        if (-273.15..=1000.0).contains(&temp) {
            Some(Celsius(temp))
        } else {
            None
        }
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}
