trait Describeable{
    fn Describe(&self) -> String;
}

struct Car {
    Brand: String,
    Speed: u32,
}

impl Describeable for Car {
    fn Describe(&self) -> String {
        format!("{} run at {} km/h" ,self.Brand , self.Speed)
    }
}

fn main() {
    let MyCar: Car = Car { Brand: String:: from("Tesla") , Speed: 150};
    println!("{}", MyCar.Describe());
}
