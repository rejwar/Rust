struct Car {
    Speed: u32,
}

fn main() {
    let mut MyCar = Car { Speed: 60 };
    MyCar.Speed = 80;
    println!("MyCar.Speed: {}", MyCar.Speed);
}
