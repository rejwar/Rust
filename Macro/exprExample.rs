macro_rules! evaluate {
    ($e: expr) => {
        let result = $e;
        println!("The expressio is evaluated to {}", result);
    };
}


fn main() {
    evaluate!(10 +5);
    evaluate!( {
        let x = 20;
        x *2
    });
}