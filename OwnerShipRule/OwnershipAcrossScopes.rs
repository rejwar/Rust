fn main() {
    let Data: String = String::from("Out Of Scope Example");
    {
        let ScopedData: String = Data; // Ownership transferred in scope
        println!("{}", ScopedData);
    }
    // println!("{}", Data); // Error! Ownership lost in the scope
}
