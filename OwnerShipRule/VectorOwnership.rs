fn main() {
    let VecData: Vec<String> = vec![String::from("Data1"), String::from("Data2")];
    let First: String = VecData[0].clone(); // Cloning to retain ownership
    println!("First Data: {}", First);
}
