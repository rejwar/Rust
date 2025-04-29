fn BorrowRules() {
    let mut name = String::from("RUst");

    let r1 = &name;
    let r2: &String = &name;

    println!("{}, {}", r1,r2);
}
