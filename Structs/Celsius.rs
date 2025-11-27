struct Temperature {
    celsius: f64,
}

impl Temperature {
    fn FromCelsius(c: f64) -> Self {
        Self { celsius: c }
    }

    fn FromFahrenheit(f: f64) -> Self {
        let c = (f - 32.0) * 5.0 / 9.0;
        Self { celsius: c }
    }

    fn ToCelsius(&self) -> f64 {
        self.celsius
    }

    fn ToFahrenheit(&self) -> f64 {
        self.celsius * 9.0 / 5.0 + 32.0
    }
}

fn main() {
    let t1 = Temperature::FromCelsius(0.0);
    println!("0 C = {} ", t1.ToFahrenheit());

    let t2 = Temperature::FromFahrenheit(212.0);
    println!(" 212 F = {} C ", t2.ToCelsius());
}
