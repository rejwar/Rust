fn main() {
    let View = MakeRef();
    println!(" {}", View);
}

fn MakeRef () -> String {
    let Temp = String::from("Temp");
    Temp
}