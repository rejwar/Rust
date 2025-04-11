fn main() {
    let OuterData: String = String::from("Outer Scope");
    {
        let InnerData: String = OuterData; // Ownership transferred to inner scope
        println!("{}", InnerData);
    }
    // println!("{}", OuterData); // Error! Ownership lost
}
