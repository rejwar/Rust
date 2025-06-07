fn main() {
    let option = Some(5);

    let mapped = option.map(|x| x * 2);
    println!("Mapped value: {:?}", mapped);

    let and_then_mapped = option.and_then(|x| Some(x * 2));
    println!("AndThen mapped value: {:?}", and_then_mapped); 
}
