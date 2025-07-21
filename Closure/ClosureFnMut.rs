fn ApplyMut<F: FnMut()> (mut  f: F) {
    f();
    f();
}

fn main() {
    let mut Count = 0;
    let mut Increament = || {
        Count += 1;
        println!("Count {}", Count); 
    };

    ApplyMut(Increament);
}