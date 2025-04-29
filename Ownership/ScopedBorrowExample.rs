fn ScopedBorrowExample() {
    let mut name = String::from("Rust");

    {
        let r1 = &name;
        println!("ReadOnly : {}", r1);
    }

    let r2 = &mut name;
    r2.push_str(" Is blazing fast");
    println!("Modified {}", r2);
}

fn main() {
    ScopedBorrowExample();
}
