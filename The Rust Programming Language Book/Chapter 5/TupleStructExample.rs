// How to use tuple structs?

struct Color(u8, u8, u8);

fn UseTupleStruct() {
    let red = Color(255, 0, 0);
    println!("Red RGB: {}, {}, {}", red.0, red.1, red.2);
}
